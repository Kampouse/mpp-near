# MPP-NEAR API Server Example

A simple REST API that wraps the mpp-near CLI for server-side testing.

## Quick Start

```bash
# Start the server
cd ~/.openclaw/workspace/mpp-near/examples
node api-server.js

# Or with custom port
PORT=8080 node api-server.js
```

## Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/health` | Health check |
| GET | `/config` | Show mpp-near config |
| GET | `/tokens` | List available tokens (?search=XXX) |
| GET | `/balance` | Check balance (?token=xxx) |
| GET | `/fund-link` | Generate funding link (?amount=X&token=XXX) |
| GET | `/handoff` | Get wallet management URL |
| POST | `/swap` | Swap tokens |
| POST | `/pay` | Send payment |
| POST | `/storage-deposit` | Register token storage |
| POST | `/create-check` | Create payment check |
| POST | `/claim-check` | Claim payment check |
| POST | `/verify` | Verify transaction |

## Examples

### Health Check
```bash
curl http://localhost:3456/health
# {"status":"ok","timestamp":"2026-03-20T15:21:59.174Z"}
```

### Check Balance
```bash
curl http://localhost:3456/balance
# {
#   "account": "5c571cf253c3edb672df980cc56078f2c455b972cc01ac34af51e95967ba6edb",
#   "near": 0.1,
#   "tokens": {"USDC": 3.714085}
# }
```

### Search Tokens
```bash
curl "http://localhost:3456/tokens?search=ZEC"
# {
#   "count": 1,
#   "total": 151,
#   "tokens": [{"chain": "ZEC", "symbol": "ZEC", "contract": "zec.omft.near", "decimals": 8}]
# }
```

### Swap Tokens
```bash
curl -X POST http://localhost:3456/swap \
  -H "Content-Type: application/json" \
  -d '{"from":"USDC","to":"ZEC","amount":"1"}'
```

### Send Payment
```bash
curl -X POST http://localhost:3456/pay \
  -H "Content-Type: application/json" \
  -d '{"to":"bob.near","amount":"0.1","token":"NEAR"}'
```

### Generate Funding Link
```bash
curl "http://localhost:3456/fund-link?amount=10&token=USDC"
```

## Configuration

Environment variables:

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | 3456 | Server port |
| `MPP_CLI` | `~/.openclaw/workspace/mpp-near/target/release/mpp-near` | Path to mpp-near binary |
| `MPP_API_KEY` | (built-in test key) | OutLayer API key |

## Architecture

```
┌─────────────────┐     ┌─────────────┐     ┌─────────────────┐
│  HTTP Client    │────▶│  API Server │────▶│  mpp-near CLI   │
│  (curl/axios)   │◀────│  (Node.js)  │◀────│  (Rust)         │
└─────────────────┘     └─────────────┘     └─────────────────┘
                                                   │
                                                   ▼
                                        ┌─────────────────┐
                                        │  OutLayer API   │
                                        │  (NEAR Intents) │
                                        └─────────────────┘
```

The API server is a thin wrapper that:
1. Spawns the mpp-near CLI as a subprocess
2. Parses the text output into structured JSON
3. Returns clean REST responses

## Use Cases

- **Testing**: Quick way to test mpp-near functionality from any HTTP client
- **Integration**: Use as a microservice in larger applications
- **Development**: Debug swap/pay operations without dealing with CLI
- **Agent Integration**: AI agents can call REST endpoints instead of shell commands

## Notes

- The server uses the API key embedded in the code for testing. For production, use environment variables.
- Swap operations require tokens to be in the intents balance. Use `/fund-link` with `dest=intents` to fund directly to intents.
- All operations are gasless (paid by OutLayer solver) except on-chain operations.
