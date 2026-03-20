# MPP-NEAR: Machine Payments Protocol for NEAR

An MPP-compatible API server that uses NEAR Intents for gasless payments.

## What is MPP?

[Machine Payments Protocol](https://mpp.dev/) is an open standard for machine-to-machine payments using HTTP 402. It enables:

- **Autonomous payments** - AI agents can pay for API access without human intervention
- **Gasless transactions** - Clients don't need gas tokens
- **Any payment method** - One protocol, multiple backends (Tempo, Lightning, NEAR, etc.)

## Architecture

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│  Client / Agent │────▶│  MPP-NEAR API   │────▶│  Your Service   │
│                 │     │                 │     │                 │
│  1. GET /api    │     │  2. 402 +       │     │                 │
│                 │◀────│     Challenge   │     │                 │
│                 │     │                 │     │                 │
│  3. Pay via     │     │                 │     │                 │
│     NEAR Intents│     │                 │     │                 │
│                 │     │                 │     │                 │
│  4. GET /api    │     │  5. Verify      │     │                 │
│     + Credential│────▶│     payment     │────▶│  6. Deliver     │
│                 │◀────│  200 + Receipt  │◀────│     resource    │
└─────────────────┘     └─────────────────┘     └─────────────────┘
                               │
                               ▼
                        ┌─────────────────┐
                        │  OutLayer API   │
                        │  (NEAR Intents) │
                        └─────────────────┘
```

## Quick Start

### Start the Server

```bash
cd ~/.openclaw/workspace/mpp-near/examples
node mpp-near-server.js
```

### Test the MPP Flow

```bash
# 1. Request protected endpoint (get 402 Challenge)
curl http://localhost:3457/api/v1/search

# 2. Parse challenge and pay
# ... pay via NEAR Intents ...

# 3. Retry with credential
curl -H 'Authorization: Payment challenge_id="xxx", intent_hash="xxx", account_id="xxx"' \
  http://localhost:3457/api/v1/search
```

### Use the Client Library

```javascript
const { MppNearClient } = require('./mpp-near-client');

const client = new MppNearClient({ 
  apiKey: 'wk_...',
  autoPay: true  // Automatically handle 402 responses
});

// Automatically pays if needed
const response = await client.fetch('http://localhost:3457/api/v1/search');
const data = await response.json();
```

## HTTP Headers

### WWW-Authenticate (Challenge)

```
WWW-Authenticate: Payment method="near-intents", recipient="0x...", amount="0.001", token="USDC", nonce="...", expires="...", challenge_id="..."
```

### Authorization (Credential)

```
Authorization: Payment challenge_id="...", intent_hash="...", account_id="..."
```

### Payment-Receipt (Confirmation)

```
Payment-Receipt: Payment receipt_id="...", challenge_id="...", account_id="...", amount="...", token="...", timestamp="...", status="confirmed"
```

## Endpoints

| Endpoint | Cost | Description |
|----------|------|-------------|
| `GET /api/v1/generate` | $0.01 USDC | Image generation |
| `GET /api/v1/search` | $0.001 USDC | Web search |
| `GET /api/v1/analyze` | $0.005 USDC | Data analysis |
| `GET /api/v1/chat` | $0.0001 USDC | Chat per token |

## Public Endpoints

| Endpoint | Description |
|----------|-------------|
| `GET /health` | Health check |
| `GET /pricing` | View pricing |
| `GET /.well-known/payment` | MPP discovery |

## MPP Compliance

This implementation follows the [MPP specification](https://mpp.dev/protocol/):

- ✅ HTTP 402 Payment Required
- ✅ WWW-Authenticate header with Challenge
- ✅ Authorization header with Credential
- ✅ Payment-Receipt header with confirmation
- ✅ `/.well-known/payment` discovery endpoint
- ✅ Idempotency (via challenge_id)
- ✅ Expiration (5-minute challenge TTL)

## Payment Method: NEAR Intents

| Feature | Details |
|---------|---------|
| **Network** | NEAR mainnet |
| **Tokens** | USDC, USDT, NEAR + 150+ cross-chain |
| **Gasless** | Yes (solver relay pays) |
| **Finality** | ~1 second |
| **Fees** | ~$0.001 per swap/transfer |

## Integration Guide

### For API Operators

1. Import the middleware:
```javascript
const { createMppMiddleware } = require('./mpp-near-server');

const pricing = {
  '/api/v1/my-endpoint': { 
    amount: '0.01', 
    token: 'USDC', 
    description: 'My API' 
  }
};

app.use(createMppMiddleware({ pricing, apiKey: 'wk_...' }));
```

2. The middleware handles:
   - 402 responses for unpaid requests
   - Payment verification
   - Receipt generation

### For Clients / Agents

1. Use the client library:
```javascript
const client = new MppNearClient({ apiKey: 'wk_...' });
const response = await client.fetch('https://api.example.com/paid-endpoint');
```

2. Or handle manually:
   - Catch 402 responses
   - Parse challenge
   - Pay via NEAR Intents
   - Retry with credential

## Comparison

| | MPP-NEAR | MPP-Tempo |
|---|---|---|
| **Network** | NEAR | Tempo blockchain |
| **Tokens** | USDC, USDT, NEAR, 150+ | TIP-20 stablecoins |
| **Gasless** | ✅ Solver relay | ✅ Fee sponsorship |
| **SDK** | Custom (this repo) | Official mpp crate |
| **Finality** | ~1s | ~500ms |
| **Standard** | MPP spec | MPP spec |

## Files

```
mpp-near/examples/
├── mpp-near-server.js   # MPP server implementation
├── mpp-near-client.js   # Auto-paying client library
├── api-server.js        # Simple REST wrapper for CLI
├── test-mpp.sh          # Test script
└── README.md            # This file
```

## Next Steps

- [ ] Add signature verification (NEP-413)
- [ ] Add session support (pay-as-you-go)
- [ ] Add Express/Hono middleware
- [ ] Add Python SDK
- [ ] Production deployment guide

## Resources

- [MPP Specification](https://mpp.dev/protocol/)
- [MPP GitHub](https://github.com/tempoxyz/mpp)
- [OutLayer API](https://api.outlayer.fastnear.com)
- [NEAR Intents](https://intents.near.org)
