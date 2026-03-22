# Cross-Chain Payments Quick Start

This is a quick reference for cross-chain payments. See [CROSS_CHAIN_PAYMENTS.md](./CROSS_CHAIN_PAYMENTS.md) for full documentation.

## 5-Minute Setup

### 1. Configure Server

```bash
export MPP_HMAC_SECRET="your-secret-here"
export MPP_RECIPIENT="jemartel.near"
export OUTLAYER_API_KEY="wk_your_api_key_here"  # Required for cross-chain
```

### 2. Start Server

```bash
cargo run --example full_server --features server
```

### 3. Test Payment

```bash
export MPP_SERVER_URL="http://localhost:3000"
export OUTLAYER_API_KEY="wk_your_api_key"
export NEAR_ACCOUNT_ID="your-account.near"

cargo run --example test_payment_client
```

## Common Commands

### Pay with Cross-Chain Tokens

```bash
# Bitcoin (on NEAR)
mpp-near pay jemartel.near 0.001 btc.omft.near --method intents

# Ethereum (on NEAR)
mpp-near pay jemartel.near 0.001 eth.omft.near --method intents

# Solana (on NEAR)
mpp-near pay jemartel.near 0.01 sol.omft.near --method intents

# Arbitrum (on NEAR)
mpp-near pay jemartel.near 0.01 arb-0xaf88d065e77c8cc2239327c5edb3a432268e5831.omft.near --method intents
```

### Withdraw to Other Chains

```bash
# Withdraw USDC to Solana
curl -X POST http://localhost:3000/withdraw \
  -H "Content-Type: application/json" \
  -d '{
    "to": "SolanaAddressHere...",
    "amount": "1.0",
    "token": "usdc",
    "chain": "solana"
  }'

# Withdraw USDC to Ethereum
curl -X POST http://localhost:3000/withdraw \
  -H "Content-Type: application/json" \
  -d '{
    "to": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
    "amount": "1.0",
    "token": "usdc",
    "chain": "ethereum"
  }'

# Withdraw Bitcoin to Bitcoin
curl -X POST http://localhost:3000/withdraw \
  -H "Content-Type: application/json" \
  -d '{
    "to": "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh",
    "amount": "0.001",
    "token": "btc.omft.near",
    "chain": "bitcoin"
  }'
```

## Supported Chains

```
near, ethereum, solana, bitcoin, arbitrum, base, polygon,
optimism, avalanche, bsc, ton, aptos, sui, starknet,
tron, stellar, dogecoin, xrp, zcash, litecoin
```

## Token Shortcuts

| Shortcut | Token ID | Decimals |
|----------|----------|----------|
| `usdc` | `17208628f...1e36133a1` | 6 |
| `usdt` | `usdt.tether-token.near` | 6 |
| `near` | `near` | 24 |

## Cross-Chain OMFT Tokens

| Token | Token ID | Original Chain |
|-------|----------|----------------|
| BTC | `btc.omft.near` | Bitcoin |
| ETH | `eth.omft.near` | Ethereum |
| SOL | `sol.omft.near` | Solana |
| ARB | `arb-0xaf88d...831.omft.near` | Arbitrum |

## Server Endpoints

```
GET  /health   - Health check
GET  /pricing  - List pricing
GET  /chains   - List supported chains
POST /withdraw - Cross-chain withdrawal
```

## Full Documentation

See [CROSS_CHAIN_PAYMENTS.md](./CROSS_CHAIN_PAYMENTS.md) for:
- Architecture overview
- Security considerations
- Troubleshooting
- Advanced examples
