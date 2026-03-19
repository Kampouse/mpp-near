# MPP-NEAR

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![License: MIT/Apache](https://img.shields.io/badge/license-MIT%20%2F%20Apache-blue.svg)](LICENSE)

NEAR payment provider for [Machine Payments Protocol (MPP)](https://mpp.dev).

Enables HTTP 402 payments with NEAR blockchain - pay for API calls with NEAR or NEP-141 tokens (USDC, etc.) in a single HTTP request.

## ✨ What's New

**Full-featured CLI tool** for gasless payments, token swaps, and agent-to-agent payment checks! 🚀

- 🪙 **Gasless Payments** - Send NEAR, USDC, USDT without gas via NEAR Intents
- 🔄 **Token Swaps** - Cross-chain swaps across 20+ blockchains
- 📝 **Payment Checks** - Agent-to-agent payments with redeemable checks
- 🖥️ **CLI Tool** - Complete command-line interface for all operations
- 🔐 **Custody Wallet** - Secure OutLayer wallet management

## 🚀 Quick Reference

```bash
# Install CLI
cargo install mpp-near --features intents

# Register gasless wallet
mpp-near register

# Fund wallet
mpp-near fund-link --amount 0.1 --token near

# Send payment (gasless!)
mpp-near pay --recipient merchant.near --amount 1

# Create payment check
mpp-near create-check --amount 10 --token usdc

# Swap tokens (gasless)
mpp-near swap --from near --to usdc --amount 1

# Start payment server
mpp-near server --recipient merchant.near --min-amount 0.001
```

## Features

- ✅ NEAR token payments
- ✅ NEP-141 token support (USDC, etc.)
- ✅ **Gasless payments via NEAR Intents** (OutLayer custody wallet)
- ✅ Agent-to-agent payment checks
- ✅ Cross-chain swaps (20+ chains)
- ✅ Automatic 402 handling middleware
- ✅ Server-side payment verification
- ✅ Axum extractors for easy integration
- ✅ Replay protection
- ✅ Balance caching
- ✅ CLI tool for all operations

## Use Cases

### 1. Monetize APIs with NEAR Payments

Accept NEAR or stablecoin payments for your API endpoints using HTTP 402:

```rust
// Your API endpoint automatically returns 402 Payment Required
// Clients are charged before accessing the content
```

### 2. Gasless Microtransactions

Send payments without worrying about gas fees:

```bash
# Send $0.01 USDC - no gas required!
mpp-near pay --recipient user.near --amount 0.01 --token usdc
```

### 3. Agent-to-Agent Commerce

AI agents can pay each other for services using payment checks:

```bash
# Agent A creates a payment check
mpp-near create-check --amount 50 --token usdc --memo "Data processing"

# Agent B claims the payment
mpp-near claim-check --check-key <check_key>
```

### 4. Cross-Chain Token Swaps

Swap tokens across 20+ blockchains without gas:

```bash
# Swap NEAR to ETH (gasless)
mpp-near swap --from near --to eth --amount 10
```

## Installation

```toml
[dependencies]
mpp-near = { git = "https://github.com/kampouse/mpp-near" }

# For gasless payments via NEAR Intents
mpp-near = { git = "https://github.com/kampouse/mpp-near", features = ["intents"] }
```

## CLI Tool

MPP-NEAR includes a powerful command-line interface for interacting with NEAR payments. The CLI supports both standard NEAR transactions and gasless operations via NEAR Intents.

### Installation

```bash
cargo install --git https://github.com/kampouse/mpp-near mpp-near --features intents
```

### Quick Start

1. **Register a gasless custody wallet:**
```bash
mpp-near register
```
This creates an OutLayer custody wallet and provides an API key for gasless transactions.

2. **Fund your wallet:**
```bash
mpp-near fund-link --amount 0.1 --token near
```
Generates a browser link to deposit NEAR into your wallet.

3. **Check your balance:**
```bash
mpp-near balance --api-key wk_...
```

4. **Send a payment (gasless!):**
```bash
mpp-near pay --recipient merchant.near --amount 1 --token near
```

### Available Commands

| Command | Description |
|---------|-------------|
| `register` | Register a new OutLayer custody wallet (gasless) |
| `pay` | Send NEAR or tokens to an account |
| `balance` | Check account balance |
| `fund-link` | Generate a funding link for your wallet |
| `handoff` | Show wallet management URL |
| `storage-deposit` | Register storage for token receipt |
| `verify` | Verify a transaction on-chain |
| `server` | Start a payment server |
| `tokens` | List available tokens (20+ chains) |
| `create-check` | Create a payment check (agent-to-agent) |
| `claim-check` | Claim a payment check |
| `swap` | Swap tokens cross-chain (gasless) |
| `config` | Show current configuration |

### CLI Examples

**Gasless Payments:**
```bash
# Send NEAR (no gas required!)
mpp-near pay --recipient friend.near --amount 0.5

# Send USDC (gasless)
mpp-near pay --recipient merchant.near --amount 10 --token usdc

# Send with memo
mpp-near pay --recipient contractor.near --amount 5 --token usdt --memo "Invoice #123"
```

**Token Swaps (20+ chains):**
```bash
# Swap NEAR to USDC (gasless)
mpp-near swap --from near --to usdc --amount 1

# List available tokens
mpp-near tokens
```

**Agent-to-Agent Payments:**
```bash
# Create a payment check
mpp-near create-check --amount 100 --token usdc --memo "API services" --expires-in 86400

# Claim a payment check (share the check_key with recipient)
mpp-near claim-check --check-key <check_key>
```

**Configuration:**
```bash
# Use config file at ~/.mpp-near/config.toml
mpp-near --config ~/.mpp-near/config.toml balance

# Show current config
mpp-near config
```

**Sample config file (`~/.mpp-near/config.toml`):**
```toml
method = "intents"

[intents]
api_key = "wk_..."
api_url = "https://api.outlayer.fastnear.com"

[standard]
account = "your-account.near"
private_key = "ed25519:..."
rpc_url = "https://rpc.mainnet.near.org"
```

### Server Mode

Start a standalone payment verification server:

```bash
mpp-near server --recipient merchant.near --min-amount 0.001 --port 3000
```

The server provides:
- `GET /` - API info
- `GET /health` - Health check
- `GET /challenge` - Create payment challenge

## Quick Start

### Option 1: Standard Client (requires NEAR for gas)

```rust
use mpp_near::client::{NearProvider, NearConfig};
use reqwest_middleware::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = NearProvider::new(
        "kampouse.near".parse()?,
        "ed25519:...".parse()?,
        "https://rpc.mainnet.near.org",
    )?;
    
    let client = ClientBuilder::new(reqwest::Client::new())
        .with(mpp_near::client::PaymentMiddleware::new(provider))
        .build();
    
    // Automatically handles 402 responses with NEAR payment
    let resp = client.get("https://api.example.com/paid").send().await?;
    
    Ok(())
}
```

### Option 2: Gasless Client (NEAR Intents)

```rust
use mpp_near::client::{IntentsProvider, IntentsConfig};
use reqwest_middleware::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from https://outlayer.fastnear.com
    let provider = IntentsProvider::new("wk_...".to_string());
    
    // Check balance (no gas needed)
    let balance = provider.check_intents_balance("usdt.tether-token.near").await?;
    println!("Balance: {}", balance);
    
    // Gasless transfer
    let recipient = "receiver.near".parse()?;
    let amount = mpp_near::NearAmount::from_near(1);
    let tx_hash = provider.transfer(&recipient, amount).await?;
    
    Ok(())
}
```

### Server (Accepting NEAR payments)

```rust
use axum::{routing::get, Router};
use mpp_near::server::{NearVerifier, VerifierConfig, NearPayment};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = VerifierConfig {
        recipient_account: "merchant.near".parse()?,
        min_amount: mpp_near::NearAmount::from_near(1),
        ..Default::default()
    };
    
    let verifier = Arc::new(NearVerifier::new(config)?);
    
    let app = Router::new()
        .route("/paid", get(|payment: NearPayment| async move {
            format!("Paid by: {} for {}", payment.payer(), payment.amount())
        }))
        .with_state(verifier);
    
    axum::serve(tokio::net::TcpListener::bind("0.0.0.0:3000").await?, app).await?;
    
    Ok(())
}
```

## NEAR Intents Features

### Gasless Transfers

```rust
let provider = IntentsProvider::new(api_key);

// No gas needed - uses solver relay
let tx_hash = provider.transfer(&recipient, amount).await?;
```

### Token Swaps (20+ chains)

```rust
// Swap wNEAR to USDC (gasless)
let result = provider.swap(
    "nep141:wrap.near",
    "nep141:17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1",
    NearAmount::from_near(1),
    None, // No slippage protection
).await?;

println!("Got {} USDC", result.amount_out);
```

### Agent-to-Agent Payments

```rust
// Create a payment check
let check = provider.create_payment_check(
    "17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1", // USDC
    NearAmount::from_usdc(10),
    Some("Payment for services"),
    Some(86400), // 24h expiry
).await?;

// Send check_key to another agent
println!("Check key: {}", check.check_key);

// Recipient claims the check
let claimed = provider.claim_payment_check(&check.check_key, None).await?;
```

### Cross-Chain Operations

Supported chains: NEAR, Ethereum, Bitcoin, Solana, Arbitrum, Base, Polygon, Optimism, Avalanche, BSC, TON, Aptos, Sui, and more.

**List all available tokens:**
```bash
mpp-near tokens
```

**Supported tokens include:**
- NEAR/wNEAR
- USDC (multi-chain)
- USDT (multi-chain)
- ETH, BTC, SOL, and 100+ more

```rust
// List available tokens
let tokens = provider.list_tokens().await?;
for token in tokens.iter().take(10) {
    println!("{} ({}) - {}", token.symbol, token.chain, token.name);
}
```

## Payment Flow

```
1. Client → Server: GET /paid-endpoint
2. Server → Client: 402 Payment Required
                    {
                      "challenge_id": "...",
                      "amount": "1000000000000000000000000",  // 1 NEAR
                      "recipient": "merchant.near",
                      "method": "near"
                    }
3. Client: Signs challenge + transfers NEAR on-chain (or via Intents)
4. Client → Server: GET /paid-endpoint
                    Authorization: Payment {"tx_hash":"...", "signature":"..."}
5. Server: Verifies on-chain transaction
6. Server → Client: 200 OK + response
```

## Gas Model

| Operation | Standard Provider | Intents Provider |
|-----------|------------------|------------------|
| Transfer NEAR | Requires gas ⛽ | **Gasless** ✨ |
| Transfer USDC/USDT | Requires gas ⛽ | **Gasless** ✨ |
| Swap tokens | Not supported | **Gasless** ✨ |
| Cross-chain | Not supported | **Gasless** ✨ |
| Payment checks | Not supported | **Gasless** ✨ |

**Why use Intents?** - Gasless transactions mean no need to hold NEAR for gas fees, instant transactions via solver relay, and support for advanced features like swaps and cross-chain operations.

## Best Practices

1. **Use payment checks for agent-to-agent payments** - No storage registration required
2. **Set expiry on checks** - 86400s (24h) default, adjust based on use case
3. **Fund with `--intents` flag** - For gasless swap/check operations
4. **Never share API keys** - Treat like private keys, store securely
5. **Use config files** - Store credentials in `~/.mpp-near/config.toml` instead of command line

## Troubleshooting

### "Storage registration required"

The recipient hasn't registered storage on the token contract. Solutions:

1. **Use a payment check** (recommended - no storage needed):
```bash
mpp-near create-check --amount 1 --token usdc --memo "Payment"
```

2. **Generate a funding link** (auto-registers storage):
```bash
mpp-near fund-link --recipient user.near --amount 0.001 --token usdc
```

3. **Ask recipient to register storage**:
```bash
mpp-near storage-deposit --account user.near --token usdc
```

### "Insufficient balance"

Your wallet needs more tokens. Generate a funding link:
```bash
mpp-near fund-link --amount 1 --token near
```

## Payment Flow
## Examples

### Using the CLI (Recommended)

```bash
# Register and fund a gasless wallet
mpp-near register
mpp-near fund-link --amount 0.1 --token near

# Send payments
mpp-near pay --recipient merchant.near --amount 1
mpp-near pay --recipient merchant.near --amount 10 --token usdc

# Start a payment server
mpp-near server --recipient merchant.near --min-amount 0.001
```

### Using Rust Code

```bash
# Standard client
NEAR_ACCOUNT_ID=kampouse.near \
NEAR_PRIVATE_KEY=ed25519:... \
cargo run --example near_client

# Server
cargo run --example near_server

# Intents client (gasless)
OUTLAYER_API_KEY=wk_... \
cargo run --example intents_client
```

## Testing

```bash
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Roadmap

- [ ] Multi-sig wallet support
- [ ] Batch payments
- [ ] Payment scheduling
- [ ] Advanced spending policies
- [ ] Mobile app support
- [ ] Web dashboard

## License

MIT OR Apache-2.0

## Credits

## Security

- Private keys never leave the client
- Replay protection via nonces
- Challenge expiration (default: 5 minutes)
- Minimum payment enforcement
- Block confirmation requirements
- **Intents:** Keys stored in TEE (Trusted Execution Environment)

## Differences from Tempo/EVM

| Feature | Tempo (EVM) | NEAR | NEAR Intents |
|---------|-------------|------|--------------|
| Address format | `0x...` (20 bytes) | `account.near` | `account.near` |
| Signature algorithm | ECDSA (secp256k1) | ED25519 | ED25519 |
| Gas model | EVM gas | NEAR Tgas | **Gasless** |
| Transaction hash | 32 bytes | 44 chars | 44 chars |
| Block time | ~12s | ~1s | ~1s |
| Cross-chain | Limited | Via bridges | **Native** |

## License

MIT OR Apache-2.0

## Credits

- Forked from [tempoxyz/mpp-rs](https://github.com/tempoxyz/mpp-rs)
- NEAR Intents powered by [OutLayer](https://outlayer.fastnear.com)

## Resources

- **Machine Payments Protocol**: https://mpp.dev
- **NEAR Blockchain**: https://near.org
- **OutLayer Dashboard**: https://outlayer.fastnear.com
- **GitHub Repository**: https://github.com/kampouse/mpp-near
- **Report Issues**: https://github.com/kampouse/mpp-near/issues
