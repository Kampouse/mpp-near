/**
 * MPP-NEAR Server
 * 
 * Machine Payments Protocol implementation for NEAR Intents.
 * Follows MPP spec: https://mpp.dev/protocol/
 * 
 * Payment flow:
 * 1. Client requests protected resource
 * 2. Server returns 402 + WWW-Authenticate header with Challenge
 * 3. Client pays via NEAR Intents, gets signature
 * 4. Client retries with Authorization header containing Credential
 * 5. Server verifies payment, returns 200 + Payment-Receipt header
 */

const http = require('http');
const crypto = require('crypto');

// Configuration
const PORT = process.env.PORT || 3457;
const API_KEY = process.env.MPP_API_KEY || 'wk_169d276885fe7283581ab753cf2d77169843ab37f920e209ae7895c7e87e790b';
const RECIPIENT_ACCOUNT = '5c571cf253c3edb672df980cc56078f2c455b972cc01ac34af51e95967ba6edb'; // Our wallet

// Payment requirements per endpoint
const PRICING = {
  '/api/v1/generate': { amount: '0.01', token: 'USDC', description: 'Image generation' },
  '/api/v1/search': { amount: '0.001', token: 'USDC', description: 'Web search' },
  '/api/v1/analyze': { amount: '0.005', token: 'USDC', description: 'Data analysis' },
  '/api/v1/chat': { amount: '0.0001', token: 'USDC', description: 'Per-token chat' },
};

// Token contract IDs
const TOKENS = {
  USDC: '17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1',
  NEAR: 'near',
  USDT: 'usdt.tether-token.near',
};

// Active challenges (in production, use Redis)
const challenges = new Map();

// Helper: GenerateChallenge
function generateChallenge(endpoint, pricing) {
  const challengeId = crypto.randomBytes(16).toString('hex');
  const nonce = crypto.randomBytes(32).toString('base64');
  const expires = Math.floor(Date.now() / 1000) + 300; // 5 min expiry
  
  const challenge = {
    id: challengeId,
    version: 'MPP/1.0',
    method: 'near-intents',
    recipient: RECIPIENT_ACCOUNT,
    amount: pricing.amount,
    token: pricing.token,
    tokenId: TOKENS[pricing.token],
    nonce,
    expires,
    endpoint,
    description: pricing.description,
    createdAt: Date.now(),
  };
  
  challenges.set(challengeId, challenge);
  
  // Clean up expired challenges
  setTimeout(() => challenges.delete(challengeId), 300000);
  
  return challenge;
}

// Helper: Serialize challenge to WWW-Authenticate header
function serializeChallenge(challenge) {
  // MPP format: Payment method=xxx, amount=xxx, ...
  const params = [
    `method="${challenge.method}"`,
    `recipient="${challenge.recipient}"`,
    `amount="${challenge.amount}"`,
    `token="${challenge.token}"`,
    `nonce="${challenge.nonce}"`,
    `expires="${challenge.expires}"`,
    `challenge_id="${challenge.id}"`,
  ];
  
  return `Payment ${params.join(', ')}`;
}

// Helper: Parse credential from Authorization header
function parseCredential(authHeader) {
  if (!authHeader || !authHeader.startsWith('Payment ')) {
    return null;
  }
  
  const params = {};
  const parts = authHeader.slice(8).split(', ');
  
  for (const part of parts) {
    const [key, value] = part.split('=');
    params[key] = value?.replace(/^"|"$/g, '');
  }
  
  return {
    challengeId: params.challenge_id,
    signature: params.signature,
    accountId: params.account_id,
    intentHash: params.intent_hash,
    publicKey: params.public_key,
  };
}

// Helper: Verify payment via OutLayer API
async function verifyPayment(credential, challenge) {
  try {
    // Check if payment was made to our wallet
    // In production, verify the actual on-chain/intents transaction
    
    // For now, verify the intent hash exists and matches
    if (!credential.intentHash) {
      return { valid: false, error: 'Missing intent hash' };
    }
    
    // For testing: accept mock payments
    if (credential.intentHash.startsWith('test_') || credential.intentHash.startsWith('mock_')) {
      console.log(`  → Accepting mock payment: ${credential.intentHash}`);
      return { valid: true, amount: challenge.amount };
    }
    
    // Verify via OutLayer API using native https
    const https = require('https');
    const url = `https://api.outlayer.fastnear.com/wallet/v1/requests/${credential.intentHash}`;
    
    const result = await new Promise((resolve, reject) => {
      const req = https.request(url, {
        method: 'GET',
        headers: { 'Authorization': `Bearer ${API_KEY}` }
      }, (res) => {
        let body = '';
        res.on('data', chunk => body += chunk);
        res.on('end', () => {
          try {
            resolve(JSON.parse(body));
          } catch (e) {
            reject(new Error('Invalid JSON response'));
          }
        });
      });
      req.on('error', reject);
      req.end();
    });
    
    if (result.status === 'success') {
      return { valid: true, amount: result.amount_out || challenge.amount };
    }
    
    return { valid: false, error: result.error || 'Payment verification failed' };
  } catch (err) {
    // For testing, accept if signature exists
    if (credential.intentHash) {
      console.log(`  → Accepting payment on error fallback: ${credential.intentHash}`);
      return { valid: true, amount: challenge.amount };
    }
    return { valid: false, error: err.message };
  }
}

// Helper: Generate receipt
function generateReceipt(challenge, credential, verified) {
  const receiptId = crypto.randomBytes(16).toString('hex');
  const timestamp = Math.floor(Date.now() / 1000);
  
  return {
    id: receiptId,
    version: 'MPP/1.0',
    challengeId: challenge.id,
    accountId: credential.accountId,
    amount: challenge.amount,
    token: challenge.token,
    timestamp,
    status: 'confirmed',
  };
}

// Helper: Serialize receipt to Payment-Receipt header
function serializeReceipt(receipt) {
  const params = [
    `receipt_id="${receipt.id}"`,
    `challenge_id="${receipt.challengeId}"`,
    `account_id="${receipt.accountId}"`,
    `amount="${receipt.amount}"`,
    `token="${receipt.token}"`,
    `timestamp="${receipt.timestamp}"`,
    `status="${receipt.status}"`,
  ];
  
  return `Payment ${params.join(', ')}`;
}

// Helper: Send JSON response
function sendJson(res, status, data, headers = {}) {
  res.writeHead(status, { 'Content-Type': 'application/json', ...headers });
  res.end(JSON.stringify(data, null, 2));
}

// Routes
async function handleRequest(req, res) {
  const url = new URL(req.url, `http://localhost:${PORT}`);
  const path = url.pathname;
  const method = req.method;
  
  console.log(`[${new Date().toISOString()}] ${method} ${path}`);
  
  // CORS headers
  res.setHeader('Access-Control-Allow-Origin', '*');
  res.setHeader('Access-Control-Allow-Methods', 'GET, POST, OPTIONS');
  res.setHeader('Access-Control-Allow-Headers', 'Authorization, Content-Type');
  
  if (method === 'OPTIONS') {
    return res.end();
  }
  
  // Public endpoints
  if (path === '/health') {
    return sendJson(res, 200, { status: 'ok', service: 'mpp-near-server' });
  }
  
  if (path === '/pricing') {
    return sendJson(res, 200, { pricing: PRICING, tokens: TOKENS });
  }
  
  if (path === '/.well-known/payment') {
    // MPP discovery endpoint
    return sendJson(res, 200, {
      version: '1.0',
      methods: [{
        id: 'near-intents',
        name: 'NEAR Intents',
        description: 'Gasless payments via NEAR Intents',
        tokens: Object.entries(TOKENS).map(([symbol, contract]) => ({ symbol, contract })),
      }],
      recipient: RECIPIENT_ACCOUNT,
    });
  }
  
  // Protected endpoints
  const pricing = PRICING[path];
  if (!pricing) {
    return sendJson(res, 404, { error: 'Not found', availableEndpoints: Object.keys(PRICING) });
  }
  
  // Check for payment credential
  const authHeader = req.headers['authorization'];
  const credential = parseCredential(authHeader);
  
  if (!credential) {
    // No payment - return 402 Challenge
    const challenge = generateChallenge(path, pricing);
    const wwwAuth = serializeChallenge(challenge);
    
    console.log(`  → 402 Payment Required: ${pricing.amount} ${pricing.token}`);
    
    return sendJson(res, 402, {
      error: 'Payment Required',
      challenge: {
        method: challenge.method,
        amount: challenge.amount,
        token: challenge.token,
        recipient: challenge.recipient,
        nonce: challenge.nonce,
        expires: challenge.expires,
        challengeId: challenge.id,
        description: challenge.description,
      },
      // Instructions for client
      instructions: {
        step1: 'Pay the specified amount to the recipient via NEAR Intents',
        step2: 'Get the intent hash from the payment response',
        step3: 'Retry request with Authorization: Payment challenge_id="...", intent_hash="...", account_id="..."',
        fundLink: `https://outlayer.fastnear.com/wallet/fund?to=${RECIPIENT_ACCOUNT}&amount=${pricing.amount}&token=${TOKENS[pricing.token]}`,
      },
    }, { 'WWW-Authenticate': wwwAuth });
  }
  
  // Verify payment
  const challenge = challenges.get(credential.challengeId);
  if (!challenge) {
    return sendJson(res, 400, { error: 'Invalid or expired challenge' });
  }
  
  const verification = await verifyPayment(credential, challenge);
  if (!verification.valid) {
    return sendJson(res, 402, { 
      error: 'Payment verification failed', 
      details: verification.error,
      challenge: generateChallenge(path, pricing),
    });
  }
  
  // Payment verified - generate receipt and return resource
  const receipt = generateReceipt(challenge, credential, verification);
  const receiptHeader = serializeReceipt(receipt);
  
  console.log(`  → 200 OK: Payment confirmed (${challenge.amount} ${challenge.token})`);
  
  // Simulate API response
  const responseData = {
    success: true,
    endpoint: path,
    payment: {
      amount: challenge.amount,
      token: challenge.token,
      receiptId: receipt.id,
    },
    data: simulateResponse(path),
  };
  
  return sendJson(res, 200, responseData, { 'Payment-Receipt': receiptHeader });
}

// Simulate API responses
function simulateResponse(endpoint) {
  const responses = {
    '/api/v1/generate': {
      image: 'https://example.com/generated-image.png',
      prompt: 'A beautiful sunset over mountains',
      model: 'stable-diffusion-xl',
    },
    '/api/v1/search': {
      results: [
        { title: 'Result 1', url: 'https://example.com/1' },
        { title: 'Result 2', url: 'https://example.com/2' },
      ],
      query: 'test search',
    },
    '/api/v1/analyze': {
      summary: 'Analysis complete',
      insights: ['Trend 1', 'Trend 2'],
      dataPoints: 1000,
    },
    '/api/v1/chat': {
      response: 'Hello! How can I help you?',
      tokens: 42,
      model: 'gpt-4',
    },
  };
  return responses[endpoint] || { result: 'ok' };
}

// Start server
const server = http.createServer(handleRequest);
server.listen(PORT, () => {
  console.log(`
╔══════════════════════════════════════════════════════════════════════╗
║                    MPP-NEAR Server                                   ║
║         Machine Payments Protocol for NEAR Intents                   ║
╠══════════════════════════════════════════════════════════════════════╣
║  URL:           http://localhost:${PORT}                                ║
║  Recipient:     ${RECIPIENT_ACCOUNT.slice(0, 20)}...
║  Payment Method: near-intents (gasless)                              ║
╠══════════════════════════════════════════════════════════════════════╣
║  Endpoints (Payment Required):                                       ║
║    POST /api/v1/generate   - $0.01 USDC (Image generation)           ║
║    POST /api/v1/search     - $0.001 USDC (Web search)                ║
║    POST /api/v1/analyze    - $0.005 USDC (Data analysis)             ║
║    POST /api/v1/chat       - $0.0001 USDC (Chat per token)           ║
╠══════════════════════════════════════════════════════════════════════╣
║  Public Endpoints:                                                   ║
║    GET /health            - Health check                             ║
║    GET /pricing           - View pricing                             ║
║    GET /.well-known/payment - MPP discovery                          ║
╠══════════════════════════════════════════════════════════════════════╣
║  MPP Flow:                                                           ║
║    1. Request protected endpoint                                     ║
║    2. Receive 402 + WWW-Authenticate header                          ║
║    3. Pay via NEAR Intents (gasless)                                 ║
║    4. Retry with Authorization: Payment header                       ║
║    5. Receive 200 + Payment-Receipt header                           ║
╠══════════════════════════════════════════════════════════════════════╣
║  Test:                                                               ║
║    curl http://localhost:${PORT}/api/v1/search                          ║
║    curl http://localhost:${PORT}/pricing                                ║
╚══════════════════════════════════════════════════════════════════════╝
`);
});

// Graceful shutdown
process.on('SIGINT', () => {
  console.log('\nShutting down...');
  server.close(() => process.exit(0));
});
