# Cross-Chain Payments with MPP-NEAR

This guide explains how to use MPP-NEAR for cross-chain payments, allowing you to receive payments on NEAR and withdraw to 20+ other blockchains including Ethereum, Solana, Bitcoin, and more.

## Table of Contents

- [Overview](#overview)
- [Supported Chains](#supported-chains)
- [Architecture](#architecture)
- [Server Setup](#server-setup)
- [Client Usage](#client-usage)
- [Cross-Chain Withdrawals](#cross-chain-withdrawals)
- [Token Reference](#token-reference)
- [Examples](#examples)

## Overview

MPP-NEAR supports **cross-chain payments** via NEAR Intents protocol and OutLayer Custody Wallet:

1. **Payment Phase**: Client pays in USDC/USDT on NEAR (gasless)
2. **Verification Phase**: Server verifies payment via MPP-1.0 protocol
3. **Withdrawal Phase**: Server withdraws to any supported chain (gasless)

### Key Benefits

- ✅ **Gasless Payments**: No gas tokens needed on any chain
- ✅ **20+ Chains**: Withdraw to Ethereum, Solana, Bitcoin, etc.
- ✅ **Atomic Cross-Chain**: Either both sides complete or nothing happens
- ✅ **OMFT Tokens**: Bridge tokens via OmniChain Functional Token standard
- ✅ **MPP-1.0 Compliant**: Full Machine Payments Protocol support

## Supported Chains

| Chain | Chain ID | Native Token | Notes |
|-------|----------|--------------|-------|
| NEAR | `near` | NEAR | Native blockchain |
| Ethereum | `ethereum` | ETH | EVM-compatible |
| Solana | `solana` | SOL | High-performance |
| Bitcoin | `bitcoin` | BTC | Store of value |
| Arbitrum | `arbitrum` | ETH | Layer 2 |
| Base | `base` | ETH | Coinbase L2 |
| Polygon | `polygon` | MATIC | EVM-compatible |
| Optimism | `optimism` | ETH | Layer 2 |
| Avalanche | `avalanche` | AVAX | EVM-compatible |
| BSC | `bsc` | BNB | Binance Smart Chain |
| TON | `ton` | TON | Telegram network |
| Aptos | `aptos` | APT | Move-based |
| Sui | `sui` | SUI | Move-based |
| StarkNet | `starknet` | ETH | ZK-rollup |
| Tron | `tron` | TRX | High-throughput |
| Stellar | `stellar` | XLM | Payments |
| Dogecoin | `dogecoin` | DOGE | Meme coin |
| XRP | `xrp` | XRP | Payments |
| Zcash | `zcash` | ZEC | Privacy |
| Litecoin | `litecoin` | LTC | Bitcoin fork |

> **Note**: More chains are added regularly. Check `/chains` endpoint for current list.

## Architecture

```
┌─────────────┐                              ┌─────────────┐
│   Client    │                              │   Server    │
│             │                              │             │
│  1. Request │────────────────────────────>│  2. Challenge │
│             │                              │  3. Payment │
│  4. Pay USDC│───> OutLayer API             │             │
│  (NEAR)     │     Gasless Intents          │             │
│             │                              │             │
│  5. Submit  │────────────────────────────>│  6. Verify  │
│     Proof   │                              │             │
│             │                              │  7. Receive  │
│  8. Response│<─────────────────────────────│     Data     │
│             │                              │             │
└─────────────┘                              │  9. Withdraw │
                                             │     USDC     │
                                             │     to       │
                                             │  Solana     │───> OutLayer API
                                             │             │    Cross-chain
                                             └─────────────┘
```

## Server Setup

### 1. Environment Variables

```bash
# Required
export MPP_HMAC_SECRET="your-secret-here"
export MPP_RECIPIENT="jemartel.near"

# Required for cross-chain withdrawals
export OUTLAYER_API_KEY="wk_your_api_key_here"

# Optional
export MPP_RPC_URL="https://rpc.mainnet.near.org"
export MPP_CHALLENGE_TTL="300"
```

### 2. Start Server

```bash
cargo run --example full_server --features server
```

### 3. Verify Cross-Chain Support

```bash
# Check supported chains
curl http://localhost:3000/chains

# Response:
{
  "supported_chains": [
    {"id": "near", "name": "NEAR Protocol", "native_token": "NEAR"},
    {"id": "ethereum", "name": "Ethereum", "native_token": "ETH"},
    {"id": "solana", "name": "Solana", "native_token": "SOL"},
    ...
  ]
}
```

## Client Usage

### Pay with Cross-Chain Tokens

The MPP client now supports paying with **any OMFT token** bridged from other chains:

```bash
# Pay with Bitcoin (bridged to NEAR)
mpp-near pay jemartel.near 0.001 btc.omft.near --method intents

# Pay with Ethereum (bridged to NEAR)
mpp-near pay jemartel.near 0.001 eth.omft.near --method intents

# Pay with Solana (bridged to NEAR)
mpp-near pay jemartel.near 0.01 sol.omft.near --method intents

# Pay with custom token (full token ID)
mpp-near pay jemartel.near 0.01 arb-0xaf88d065e77c8cc2239327c5edb3a432268e5831.omft.near --method intents
```

### Test Client with Cross-Chain

```bash
# Set environment variables
export MPP_SERVER_URL="http://localhost:3000"
export OUTLAYER_API_KEY="wk_your_api_key"
export NEAR_ACCOUNT_ID="your-account.near"

# Run test
cargo run --example test_payment_client
```

## Cross-Chain Withdrawals

After receiving payment on NEAR, you can withdraw to any supported chain.

### Withdrawal API Endpoint

**POST** `/withdraw`

```json
{
  "to": "destination_address",
  "amount": "1.5",
  "token": "usdc",
  "chain": "solana"
}
```

### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `to` | string | Yes | Destination address (NEAR account or other chain address) |
| `amount` | string | Yes | Amount in human-readable format (e.g., "1.5") |
| `token` | string | Yes | Token to withdraw ("usdc", "usdt", "near", or custom token ID) |
| `chain` | string | No | Destination chain (default: "near") |

### Supported Tokens

- **Shortcut tokens**: `usdc`, `usdt`, `near`
- **Custom tokens**: Any token ID (e.g., `btc.omft.near`, `eth.omft.near`)

### Response

Success (200):
```json
{
  "status": "success",
  "message": "Cross-chain withdrawal initiated",
  "withdrawal": {
    "to": "DestinationAddressHere...",
    "amount": "1.5",
    "token": "usdc",
    "chain": "solana",
    "transaction": "uuid-request-id"
  }
}
```

Error (400/500):
```json
{
  "type": "https://mpp.dev/problems/withdrawal-failed",
  "title": "Withdrawal Failed",
  "detail": "OutLayer API error: ...",
  "status": 500
}
```

## Token Reference

### OMFT Tokens (Cross-Chain)

OMFT (OmniChain Functional Token) allows tokens from other chains to be used on NEAR:

| Token | Token ID | Original Chain | Decimals |
|-------|----------|----------------|----------|
| BTC | `btc.omft.near` | Bitcoin | 8 |
| ETH | `eth.omft.near` | Ethereum | 18 |
| SOL | `sol.omft.near` | Solana | 9 |
| ARB | `arb-0xaf88d065e77c8cc2239327c5edb3a432268e5831.omft.near` | Arbitrum | 18 |
| Base | `base.omft.near` | Base | 18 |

### Native NEAR Tokens

| Token | Token ID | Decimals | Notes |
|-------|----------|----------|-------|
| NEAR | `near` | 24 | Native token |
| wNEAR | `wrap.near` | 24 | Wrapped NEAR |
| USDC | `17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1` | 6 | USDC on NEAR |
| USDT | `usdt.tether-token.near` | 6 | USDT on NEAR |

## Examples

### Example 1: Pay with USDC, Withdraw to Solana

```bash
# 1. Client makes request
curl http://localhost:3000/api/v1/ping

# 2. Client receives 402 challenge with WWW-Authenticate header

# 3. Client pays USDC via OutLayer API
curl -X POST https://api.outlayer.fastnear.com/wallet/v1/intents/withdraw \
  -H "Authorization: Bearer $OUTLAYER_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "to": "jemartel.near",
    "amount": "1000000",
    "token": "17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1",
    "chain": "near"
  }'

# 4. Client creates credential with proof (request_id)
# 5. Client retries request with Authorization header
# 6. Server verifies payment and returns data

# 7. Server withdraws to Solana
curl -X POST http://localhost:3000/withdraw \
  -H "Content-Type: application/json" \
  -d '{
    "to": "SolanaAddressHere...",
    "amount": "1.0",
    "token": "usdc",
    "chain": "solana"
  }'
```

### Example 2: Pay with Bitcoin, Withdraw to Ethereum

```bash
# 1. Client pays with Bitcoin (OMFT on NEAR)
mpp-near pay jemartel.near 0.001 btc.omft.near --method intents

# 2. Server receives payment in Bitcoin (on NEAR)

# 3. Server withdraws to Ethereum
curl -X POST http://localhost:3000/withdraw \
  -H "Content-Type: application/json" \
  -d '{
    "to": "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb",
    "amount": "0.001",
    "token": "btc.omft.near",
    "chain": "ethereum"
  }'
```

### Example 3: MPP Server with Auto-Withdrawal

You can modify the server to automatically withdraw to other chains after payment:

```rust
// In api_handler, after payment verification:
if verified {
    // Auto-withdraw to Ethereum
    let withdraw_req = WithdrawRequest {
        to: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb".to_string(),
        amount: pricing.amount.clone(),
        token: pricing.currency.clone(),
        chain: "ethereum".to_string(),
    };

    // Trigger async withdrawal
    let state_clone = state.clone();
    tokio::spawn(async move {
        let _ = withdraw_cross_chain(
            State(state_clone),
            Json(withdraw_req),
        ).await;
    });

    // Return response immediately
    return handle_paid_request(path).await;
}
```

## Security Considerations

1. **API Key Security**: Never commit `OUTLAYER_API_KEY` to git
2. **HMAC Secret**: Use strong random secret for challenge binding
3. **Recipient Verification**: Always verify payment recipient matches expected
4. **Amount Validation**: Validate amounts are positive and reasonable
5. **UUID Verification**: Verify payment proofs are valid UUIDs (not mocks)

## Troubleshooting

### OutLayer API Errors

**"has no storage"**
```bash
# Solution: Register storage for recipient
curl -X POST https://api.outlayer.fastnear.com/wallet/v1/storage-deposit \
  -H "Authorization: Bearer $OUTLAYER_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"token": "17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1"}'
```

**"insufficient_balance"**
```bash
# Solution: Fund your OutLayer wallet
# Generate a funding link at: https://outlayer.fastnear.com/wallet/fund
```

### Withdrawal Failures

1. **Invalid Chain ID**: Check `/chains` endpoint for supported chains
2. **Invalid Address**: Verify address format for destination chain
3. **Unsupported Token**: Use `/tokens` endpoint to check supported tokens
4. **Network Issues**: Check OutLayer API status

## Resources

- [MPP-1.0 Spec](https://github.com/machine-payments-protocol/mpp-spec)
- [NEAR Intents Documentation](https://near.org/intents)
- [OutLayer Custody Wallet](https://skills.outlayer.ai/agent-custody/SKILL.md)
- [OMFT Standard](https://omft.org)

## License

MIT
