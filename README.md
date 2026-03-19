# MPP-NEAR

NEAR payment provider for [Machine Payments Protocol (MPP)](https://mpp.dev).

Enables HTTP 402 payments with NEAR blockchain - pay for API calls with NEAR or NEP-141 tokens (USDC, etc.) in a single HTTP request.

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

## Installation

```toml
[dependencies]
mpp-near = { git = "https://github.com/kampouse/mpp-near" }

# For gasless payments via NEAR Intents
mpp-near = { git = "https://github.com/kampouse/mpp-near", features = ["intents"] }
```

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
| Transfer NEAR | Requires gas | **Gasless** |
| Transfer USDC | Requires gas | **Gasless** |
| Swap tokens | Not supported | **Gasless** |
| Cross-chain | Not supported | **Gasless** |
| Payment checks | Not supported | **Gasless** |

## Examples

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
