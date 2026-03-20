/**
 * MPP-NEAR Client
 * 
 * Automatically handles MPP 402 responses for NEAR Intents payments.
 * Drop-in replacement for fetch() that pays and retries.
 * 
 * Usage:
 *   const client = new MppNearClient({ apiKey: 'wk_...' });
 *   const response = await client.fetch('http://localhost:3457/api/v1/search');
 */

const { spawn } = require('child_process');
const path = require('path');
const http = require('http');
const https = require('https');

class MppNearClient {
  constructor(options = {}) {
    this.apiKey = options.apiKey || process.env.MPP_API_KEY || 'wk_169d276885fe7283581ab753cf2d77169843ab37f920e209ae7895c7e87e790b';
    this.mppCli = options.mppCli || path.join(
      require('os').homedir(),
      '.openclaw/workspace/mpp-near/target/release/mpp-near'
    );
    this.autoPay = options.autoPay !== false; // Default: true
    this.maxRetries = options.maxRetries || 3;
    this.verbose = options.verbose || false;
  }

  log(...args) {
    if (this.verbose) {
      console.log('[MPP-NEAR]', ...args);
    }
  }

  /**
   * Make a payment-aware fetch request
   */
  async fetch(url, options = {}) {
    let retries = 0;
    
    while (retries < this.maxRetries) {
      const response = await this._fetch(url, options);
      
      // Not a 402 - return as-is
      if (response.status !== 402) {
        return response;
      }
      
      // 402 but auto-pay disabled
      if (!this.autoPay) {
        return response;
      }
      
      this.log(`402 Payment Required, handling payment...`);
      
      // Parse challenge
      const wwwAuth = response.headers.get('www-authenticate');
      const challenge = this.parseChallenge(response);
      
      if (!challenge) {
        this.log('No valid challenge found');
        return response;
      }
      
      this.log(`Challenge: ${challenge.amount} ${challenge.token} to ${challenge.recipient}`);
      
      // Pay
      const payment = await this.pay(challenge);
      
      if (!payment.success) {
        this.log(`Payment failed: ${payment.error}`);
        return response;
      }
      
      this.log(`Payment successful: ${payment.intentHash}`);
      
      // Retry with credential
      options.headers = {
        ...options.headers,
        'Authorization': this.buildCredential(challenge, payment),
      };
      
      retries++;
    }
    
    throw new Error(`Max retries (${this.maxRetries}) exceeded`);
  }

  /**
   * Internal fetch implementation
   */
  async _fetch(url, options = {}) {
    return new Promise((resolve, reject) => {
      const parsedUrl = new URL(url);
      const lib = parsedUrl.protocol === 'https:' ? https : http;
      
      const reqOptions = {
        hostname: parsedUrl.hostname,
        port: parsedUrl.port || (parsedUrl.protocol === 'https:' ? 443 : 80),
        path: parsedUrl.pathname + parsedUrl.search,
        method: options.method || 'GET',
        headers: options.headers || {},
      };
      
      const req = lib.request(reqOptions, (res) => {
        let body = '';
        res.on('data', chunk => body += chunk);
        res.on('end', () => {
          resolve({
            status: res.statusCode,
            headers: {
              get: (name) => res.headers[name.toLowerCase()],
            },
            json: async () => JSON.parse(body),
            text: async () => body,
            body,
          });
        });
      });
      
      req.on('error', reject);
      
      if (options.body) {
        req.write(options.body);
      }
      
      req.end();
    });
  }

  /**
   * Parse challenge from 402 response
   */
  parseChallenge(response) {
    // Try to parse from JSON body first
    try {
      const body = typeof response.body === 'string' 
        ? JSON.parse(response.body) 
        : response.body;
      
      if (body.challenge) {
        return {
          id: body.challenge.challengeId,
          method: body.challenge.method,
          amount: body.challenge.amount,
          token: body.challenge.token,
          recipient: body.challenge.recipient,
          nonce: body.challenge.nonce,
          expires: body.challenge.expires,
        };
      }
    } catch (e) {
      // Not JSON, try header
    }
    
    // Parse from WWW-Authenticate header
    const wwwAuth = response.headers.get('www-authenticate');
    if (!wwwAuth || !wwwAuth.startsWith('Payment ')) {
      return null;
    }
    
    const params = {};
    const parts = wwwAuth.slice(8).split(', ');
    
    for (const part of parts) {
      const [key, value] = part.split('=');
      params[key] = value?.replace(/^"|"$/g, '');
    }
    
    return {
      id: params.challenge_id,
      method: params.method,
      amount: params.amount,
      token: params.token,
      recipient: params.recipient,
      nonce: params.nonce,
      expires: parseInt(params.expires),
    };
  }

  /**
   * Execute payment via mpp-near CLI
   */
  async pay(challenge) {
    return new Promise((resolve) => {
      const args = [
        'pay',
        '--to', challenge.recipient,
        '--amount', challenge.amount,
        '--token', challenge.token,
        '--api-key', this.apiKey,
      ];
      
      this.log(`Running: ${this.mppCli} ${args.join(' ')}`);
      
      const proc = spawn(this.mppCli, args, {
        env: { ...process.env, NO_COLOR: '1' }
      });
      
      let stdout = '';
      let stderr = '';
      
      proc.stdout.on('data', (data) => stdout += data);
      proc.stderr.on('data', (data) => stderr += data);
      
      proc.on('close', (code) => {
        if (code === 0) {
          // Extract intent hash from output
          const hashMatch = stdout.match(/(?:intent_hash|hash|Intent):\s*([A-Za-z0-9]+)/i);
          const intentHash = hashMatch ? hashMatch[1] : `paid_${Date.now()}`;
          
          resolve({
            success: true,
            intentHash,
            output: stdout,
          });
        } else {
          resolve({
            success: false,
            error: stderr || stdout || `Exit code ${code}`,
          });
        }
      });
      
      proc.on('error', (err) => {
        resolve({ success: false, error: err.message });
      });
    });
  }

  /**
   * Build Authorization header with credential
   */
  buildCredential(challenge, payment) {
    const params = [
      `challenge_id="${challenge.id}"`,
      `intent_hash="${payment.intentHash}"`,
      `account_id="${this.apiKey.slice(0, 8)}..."`,
    ];
    
    return `Payment ${params.join(', ')}`;
  }

  /**
   * Get balance
   */
  async getBalance() {
    return new Promise((resolve, reject) => {
      const proc = spawn(this.mppCli, ['balance', '--api-key', this.apiKey], {
        env: { ...process.env, NO_COLOR: '1' }
      });
      
      let stdout = '';
      proc.stdout.on('data', (data) => stdout += data);
      
      proc.on('close', (code) => {
        if (code === 0) {
          // Parse balance
          const nearMatch = stdout.match(/Balance:\s*([\d.]+)\s*NEAR/);
          const usdcMatch = stdout.match(/USDC:\s*([\d.]+)/);
          
          resolve({
            near: nearMatch ? parseFloat(nearMatch[1]) : 0,
            usdc: usdcMatch ? parseFloat(usdcMatch[1]) : 0,
            raw: stdout,
          });
        } else {
          reject(new Error('Failed to get balance'));
        }
      });
    });
  }
}

// CLI usage
if (require.main === module) {
  const client = new MppNearClient({ verbose: true });
  const url = process.argv[2] || 'http://localhost:3457/api/v1/search';
  
  console.log(`Fetching: ${url}\n`);
  
  client.fetch(url)
    .then(async (res) => {
      console.log(`Status: ${res.status}`);
      
      if (res.status === 200) {
        const receipt = res.headers.get('payment-receipt');
        if (receipt) {
          console.log(`Receipt: ${receipt}`);
        }
      }
      
      const data = await res.json();
      console.log('\nResponse:');
      console.log(JSON.stringify(data, null, 2));
    })
    .catch(err => {
      console.error('Error:', err.message);
      process.exit(1);
    });
}

module.exports = { MppNearClient };
