#!/usr/bin/env node
/**
 * MPP-NEAR API Server Example
 * 
 * A REST API that wraps mpp-near CLI for server-side testing.
 * Run: node api-server.js
 * Test: curl http://localhost:3456/balance
 */

const { spawn } = require('child_process');
const http = require('http');
const url = require('url');

// Configuration
const PORT = process.env.PORT || 3456;
const MPP_CLI = process.env.MPP_CLI || require('path').join(
  require('os').homedir(),
  '.openclaw/workspace/mpp-near/target/release/mpp-near'
);
const API_KEY = process.env.MPP_API_KEY || 'wk_169d276885fe7283581ab753cf2d77169843ab37f920e209ae7895c7e87e790b';

// Helper: Run mpp-near CLI
function runMpp(args, timeout = 30000) {
  return new Promise((resolve, reject) => {
    const timeoutId = setTimeout(() => {
      proc.kill();
      reject(new Error('Timeout'));
    }, timeout);

    const proc = spawn(MPP_CLI, [...args, '--api-key', API_KEY], {
      env: { ...process.env, NO_COLOR: '1' }
    });

    let stdout = '';
    let stderr = '';

    proc.stdout.on('data', (data) => stdout += data);
    proc.stderr.on('data', (data) => stderr += data);

    proc.on('close', (code) => {
      clearTimeout(timeoutId);
      if (code === 0) {
        resolve(stdout.trim());
      } else {
        reject(new Error(stderr || stdout || `Exit code ${code}`));
      }
    });

    proc.on('error', (err) => {
      clearTimeout(timeoutId);
      reject(err);
    });
  });
}

// Helper: Parse balance output
function parseBalance(text) {
  const result = { raw: text };
  
  // Parse account
  const accountMatch = text.match(/Account:\s*([a-f0-9]+)/);
  if (accountMatch) result.account = accountMatch[1];
  
  // Parse NEAR balance
  const nearMatch = text.match(/Balance:\s*([\d.]+)\s*NEAR/);
  if (nearMatch) result.near = parseFloat(nearMatch[1]);
  
  // Parse token balances (e.g., "USDC:    3.714085")
  // Only match lines that look like "SYMBOL:    number"
  const lines = text.split('\n');
  result.tokens = {};
  for (const line of lines) {
    const match = line.match(/^\s*([A-Z][A-Z0-9]+):\s+([\d.]+)\s*$/);
    if (match && match[1] !== 'Balance') {
      result.tokens[match[1]] = parseFloat(match[2]);
    }
  }
  
  return result;
}

// Helper: Parse swap output
function parseSwap(text) {
  const result = { raw: text };
  
  // Parse amount out
  const amountMatch = text.match(/(?:amount_out|Amount out|Got):\s*([\d.]+)/i);
  if (amountMatch) result.amountOut = amountMatch[1];
  
  // Parse intent hash
  const hashMatch = text.match(/(?:intent_hash|hash|tx):\s*([A-Za-z0-9]+)/i);
  if (hashMatch) result.intentHash = hashMatch[1];
  
  // Check for success indicators
  result.success = text.includes('✓') || text.includes('success') || text.includes('Swapped');
  
  return result;
}

// Helper: Parse tokens list
function parseTokens(text) {
  const tokens = [];
  const lines = text.split('\n');
  let currentChain = null;
  
  for (const line of lines) {
    // Chain header (e.g., "NEAR", "ETH")
    if (line.match(/^[A-Z]{2,}$/) && !line.includes(':')) {
      currentChain = line.trim();
      continue;
    }
    
    // Token line (e.g., "BTC    - nbtc.bridge.near (8 decimals)")
    const tokenMatch = line.match(/^\s*(\w+)\s*-\s*([^\s]+)\s*\((\d+)\s*decimals\)/);
    if (tokenMatch) {
      tokens.push({
        chain: currentChain,
        symbol: tokenMatch[1],
        contract: tokenMatch[2],
        decimals: parseInt(tokenMatch[3])
      });
    }
  }
  
  return tokens;
}

// Helper: Send JSON response
function sendJson(res, status, data) {
  res.writeHead(status, { 'Content-Type': 'application/json' });
  res.end(JSON.stringify(data, null, 2));
}

// Routes
const routes = {
  // GET /health - Health check
  'GET /health': async (req, res, params) => {
    sendJson(res, 200, { status: 'ok', timestamp: new Date().toISOString() });
  },

  // GET /config - Show mpp-near config
  'GET /config': async (req, res, params) => {
    try {
      const output = await runMpp(['config']);
      sendJson(res, 200, { config: output });
    } catch (err) {
      sendJson(res, 500, { error: err.message });
    }
  },

  // GET /tokens - List available tokens
  'GET /tokens': async (req, res, params) => {
    try {
      const output = await runMpp(['tokens']);
      const tokens = parseTokens(output);
      const filtered = params.search 
        ? tokens.filter(t => t.symbol.toLowerCase().includes(params.search.toLowerCase()))
        : tokens;
      sendJson(res, 200, { 
        count: filtered.length,
        total: tokens.length,
        tokens: filtered
      });
    } catch (err) {
      sendJson(res, 500, { error: err.message });
    }
  },

  // GET /balance - Check balance (optional ?token=xxx)
  'GET /balance': async (req, res, params) => {
    try {
      const args = ['balance'];
      if (params.token) {
        args.push('--token', params.token);
      }
      const output = await runMpp(args);
      const parsed = parseBalance(output);
      sendJson(res, 200, parsed);
    } catch (err) {
      sendJson(res, 500, { error: err.message });
    }
  },

  // GET /fund-link - Generate funding link
  'GET /fund-link': async (req, res, params) => {
    try {
      const args = ['fund-link'];
      if (params.amount) {
        args.push('--amount', params.amount);
      }
      if (params.token) {
        args.push('--token', params.token);
      }
      const output = await runMpp(args);
      sendJson(res, 200, { fundLink: output });
    } catch (err) {
      sendJson(res, 500, { error: err.message });
    }
  },

  // GET /handoff - Get wallet management URL
  'GET /handoff': async (req, res, params) => {
    try {
      const output = await runMpp(['handoff']);
      sendJson(res, 200, { handoffUrl: output });
    } catch (err) {
      sendJson(res, 500, { error: err.message });
    }
  },

  // POST /swap - Swap tokens (body: { from, to, amount })
  'POST /swap': async (req, res, params, body) => {
    try {
      const { from, to, amount, minOut } = body;
      if (!from || !to || !amount) {
        return sendJson(res, 400, { error: 'Missing required: from, to, amount' });
      }
      
      const args = ['swap', '--from', from, '--to', to, '--amount', amount];
      if (minOut) {
        args.push('--min-out', minOut);
      }
      
      const output = await runMpp(args, 60000); // 60s timeout for swaps
      const parsed = parseSwap(output);
      sendJson(res, 200, parsed);
    } catch (err) {
      sendJson(res, 500, { error: err.message });
    }
  },

  // POST /pay - Send payment (body: { to, amount, token? })
  'POST /pay': async (req, res, params, body) => {
    try {
      const { to, amount, token } = body;
      if (!to || !amount) {
        return sendJson(res, 400, { error: 'Missing required: to, amount' });
      }
      
      const args = ['pay', '--to', to, '--amount', amount];
      if (token) {
        args.push('--token', token);
      }
      
      const output = await runMpp(args, 60000);
      sendJson(res, 200, { success: true, result: output });
    } catch (err) {
      sendJson(res, 500, { error: err.message });
    }
  },

  // POST /storage-deposit - Register token storage
  'POST /storage-deposit': async (req, res, params, body) => {
    try {
      const { token, account } = body;
      if (!token) {
        return sendJson(res, 400, { error: 'Missing required: token' });
      }
      
      const args = ['storage-deposit', '--token', token];
      if (account) {
        args.push('--account', account);
      }
      
      const output = await runMpp(args);
      sendJson(res, 200, { success: true, result: output });
    } catch (err) {
      sendJson(res, 500, { error: err.message });
    }
  },

  // POST /create-check - Create payment check
  'POST /create-check': async (req, res, params, body) => {
    try {
      const { token, amount, memo } = body;
      if (!token || !amount) {
        return sendJson(res, 400, { error: 'Missing required: token, amount' });
      }
      
      const args = ['create-check', '--token', token, '--amount', amount];
      if (memo) {
        args.push('--memo', memo);
      }
      
      const output = await runMpp(args);
      sendJson(res, 200, { success: true, result: output });
    } catch (err) {
      sendJson(res, 500, { error: err.message });
    }
  },

  // POST /claim-check - Claim payment check
  'POST /claim-check': async (req, res, params, body) => {
    try {
      const { checkKey, amount } = body;
      if (!checkKey) {
        return sendJson(res, 400, { error: 'Missing required: checkKey' });
      }
      
      const args = ['claim-check', '--check-key', checkKey];
      if (amount) {
        args.push('--amount', amount);
      }
      
      const output = await runMpp(args);
      sendJson(res, 200, { success: true, result: output });
    } catch (err) {
      sendJson(res, 500, { error: err.message });
    }
  },

  // POST /verify - Verify transaction
  'POST /verify': async (req, res, params, body) => {
    try {
      const { txHash } = body;
      if (!txHash) {
        return sendJson(res, 400, { error: 'Missing required: txHash' });
      }
      
      const output = await runMpp(['verify', '--tx-hash', txHash]);
      sendJson(res, 200, { success: true, result: output });
    } catch (err) {
      sendJson(res, 500, { error: err.message });
    }
  },
};

// Parse request body
function parseBody(req) {
  return new Promise((resolve, reject) => {
    let body = '';
    req.on('data', chunk => body += chunk);
    req.on('end', () => {
      try {
        resolve(body ? JSON.parse(body) : {});
      } catch (err) {
        reject(new Error('Invalid JSON'));
      }
    });
    req.on('error', reject);
  });
}

// Server handler
async function handler(req, res) {
  const parsedUrl = url.parse(req.url, true);
  const path = parsedUrl.pathname;
  const params = parsedUrl.query;
  const method = req.method;
  const routeKey = `${method} ${path}`;

  console.log(`[${new Date().toISOString()}] ${routeKey}`);

  // Find matching route
  const route = routes[routeKey];
  if (!route) {
    return sendJson(res, 404, { 
      error: 'Not found',
      availableRoutes: Object.keys(routes)
    });
  }

  try {
    let body = {};
    if (method === 'POST' || method === 'PUT') {
      body = await parseBody(req);
    }
    await route(req, res, params, body);
  } catch (err) {
    console.error('Handler error:', err);
    sendJson(res, 500, { error: err.message });
  }
}

// Start server
const server = http.createServer(handler);
server.listen(PORT, () => {
  console.log(`
╔════════════════════════════════════════════════════════════╗
║              MPP-NEAR API Server                           ║
╠════════════════════════════════════════════════════════════╣
║  URL:  http://localhost:${PORT}                              ║
║  CLI:  ${MPP_CLI.split('/').slice(-2).join('/')}
╠════════════════════════════════════════════════════════════╣
║  Endpoints:                                                ║
║    GET  /health           Health check                     ║
║    GET  /config           Show mpp-near config             ║
║    GET  /tokens           List available tokens            ║
║    GET  /balance          Check balance (?token=xxx)       ║
║    GET  /fund-link        Generate funding link            ║
║    GET  /handoff          Get wallet management URL        ║
║    POST /swap             Swap tokens {from,to,amount}     ║
║    POST /pay              Send payment {to,amount,token?}  ║
║    POST /storage-deposit  Register storage {token}         ║
║    POST /create-check     Create check {token,amount}      ║
║    POST /claim-check      Claim check {checkKey}           ║
║    POST /verify           Verify tx {txHash}               ║
╠════════════════════════════════════════════════════════════╣
║  Test:                                                     ║
║    curl http://localhost:${PORT}/health                      ║
║    curl http://localhost:${PORT}/balance                     ║
║    curl http://localhost:${PORT}/tokens?search=ZEC          ║
║    curl -X POST http://localhost:${PORT}/swap \\              ║
║      -H "Content-Type: application/json" \\                 ║
║      -d '{"from":"USDC","to":"ZEC","amount":"0.5"}'        ║
╚════════════════════════════════════════════════════════════╝
`);
});

// Graceful shutdown
process.on('SIGINT', () => {
  console.log('\nShutting down...');
  server.close(() => process.exit(0));
});
