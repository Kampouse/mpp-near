# mpp-near - NEAR Payment Provider for MPP

NEAR payment implementation for Machine Payments Protocol (MPP). Enables HTTP 402 payments with NEAR blockchain - gasless via NEAR Intents or standard on-chain transactions.

## What This Does

- **HTTP 402 payments** - Pay for API calls with NEAR/USDC/USDT
- **Gasless payments** - Use NEAR Intents (no NEAR needed for gas)
- **Standard payments** - Direct on-chain transactions
- **Agent-to-agent payments** - Payment checks for AI agents
- **Cross-chain swaps** - 20+ chains supported (Intents)

## Installation

### As CLI Tool

```bash
cd ~/.openclaw/workspace/mpp-near
cargo install --path .

# Now available globally
mpp-near --help
```

### As Library

```toml
# In your Cargo.toml
[dependencies]
mpp-near = { git = "https://github.com/Kampouse/mpp-near", features = ["intents"] }
```

## Quick Start

### 1. Configure

Create `~/.mpp-near/config.toml`:

```toml
method = "intents"

[intents]
api_key = "wk_..."  # Get from https://outlayer.fastnear.com

# Optional: Standard provider
[standard]
account = "kampouse.near"
private_key = "ed25519:..."
rpc_url = "https://rpc.mainnet.near.org"
```

### 2. Send Payment

```bash
# Gasless payment (Intents)
mpp-near pay --recipient merchant.near --amount 1

# Standard payment (requires gas)
mpp-near pay \
  --recipient merchant.near \
  --amount 1 \
  --method standard \
  --account kampouse.near \
  --private-key ed25519:...
```

## Commands

### `pay` - Send Payment

Send NEAR or tokens to a recipient.

```bash
# Send 1 NEAR (gasless)
mpp-near pay --recipient merchant.near --amount 1

# Send 10 USDC (gasless)
mpp-near pay --recipient merchant.near --amount 10 --token usdc

# Send with memo
mpp-near pay --recipient merchant.near --amount 1 --memo "Invoice #123"

# Standard payment (requires NEAR for gas)
mpp-near pay \
  --recipient merchant.near \
  --amount 1 \
  --method standard \
  --account kampouse.near \
  --private-key ed25519:5Kd3...
```

**Options:**
- `--recipient` - Account ID to receive payment (required)
- `--amount` - Amount in NEAR (e.g., "1.5") (required)
- `--token` - Token to send: near, usdc, usdt (default: near)
- `--memo` - Optional memo
- `--method` - Payment method: standard or intents (default: intents)
- `--api-key` - OutLayer API key for intents
- `--account` - NEAR account for standard
- `--private-key` - Private key for standard

### `balance` - Check Balance

Check account balance.

```bash
# Intents wallet balance
mpp-near balance --api-key wk_...

# Standard account balance
mpp-near balance --account kampouse.near --method standard
```

### `tokens` - List Available Tokens

List tokens available for Intents (20+ chains).

```bash
mpp-near tokens --api-key wk_...
```

Output:
```
✓ Found 156 tokens

NEAR    - NEAR Protocol (near)
ETH     - Ethereum (ethereum)
USDC    - USD Coin (ethereum)
BTC     - Bitcoin (bitcoin)
SOL     - Solana (solana)
...
```

### `config` - Show Configuration

Display current configuration.

```bash
mpp-near config
```

## Payment Methods

### Intents (Gasless) - Recommended

**Pros:**
- ✅ No gas required
- ✅ Instant (solver pays gas)
- ✅ Cross-chain swaps (20+ chains)
- ✅ Agent-to-agent payment checks

**Cons:**
- ❌ Requires OutLayer API key
- ❌ Custody wallet (not your keys)

**When to use:**
- No NEAR for gas
- Want gasless operations
- Need cross-chain swaps
- Building AI agents

**Get API key:** https://outlayer.fastnear.com

### Standard (On-chain)

**Pros:**
- ✅ Works with any NEAR wallet
- ✅ Direct on-chain transactions
- ✅ Full control (your keys)

**Cons:**
- ❌ Requires NEAR for gas fees (~0.00005 NEAR/tx)
- ❌ Slower (~1-2 seconds)

**When to use:**
- Have NEAR for gas
- Want full control
- Building decentralized apps

## Use Cases

### 1. Pay for API Access

```bash
# Client pays for API call
mpp-near pay \
  --recipient api-provider.near \
  --amount 0.1 \
  --method intents \
  --api-key $OUTLAYER_API_KEY
```

### 2. Gasless Token Swap

```bash
# Swap 1 NEAR to USDC (gasless)
# (Would need to implement swap command)
```

### 3. Agent-to-Agent Payment

```bash
# Create payment check for another agent
# (Would need to implement create-check command)

# Agent claims the check
# (Would need to implement claim-check command)
```

### 4. Automated Payments

```bash
#!/bin/bash
# Cron job to pay for services daily

API_KEY="wk_..."
RECIPIENT="service-provider.near"

mpp-near pay \
  --recipient $RECIPIENT \
  --amount 0.5 \
  --method intents \
  --api-key $API_KEY
```

## Integration Examples

### Rust Library

```rust
use mpp_near::client::{IntentsProvider, NearProvider};
use mpp_near::types::{AccountId, NearAmount};

// Gasless payments
let provider = IntentsProvider::new("wk_...".to_string());
let recipient = AccountId::new("merchant.near")?;
let amount = NearAmount::from_near(1);

let tx_hash = provider.transfer(&recipient, amount).await?;
println!("Payment sent: {}", tx_hash);

// Standard payments
let provider = NearProvider::new(
    "kampouse.near".parse()?,
    "ed25519:...".to_string(),
    "https://rpc.mainnet.near.org",
)?;
let tx_hash = provider.transfer(&recipient, amount).await?;
```

### HTTP Client (Middleware)

```rust
use mpp_near::client::{IntentsProvider, PaymentMiddleware};
use reqwest_middleware::ClientBuilder;

let provider = IntentsProvider::new("wk_...".to_string());

let client = ClientBuilder::new(reqwest::Client::new())
    .with(PaymentMiddleware::new(provider))
    .build();

// Automatically handles HTTP 402 responses
let resp = client.get("https://api.example.com/paid-endpoint")
    .send()
    .await?;
```

### MCP Tool (AI Agents)

```rust
// For MCP tools that require payment
// Use _meta.org.paymentauth/credential field

{
  "method": "tools/call",
  "params": {
    "name": "expensive-api",
    "_meta": {
      "org.paymentauth/credential": {
        "challenge": {...},
        "payload": {
          "tx_hash": "...",
          "signature": "..."
        }
      }
    }
  }
}
```

## Configuration

### Config File Location

Default: `~/.mpp-near/config.toml`

Custom: `mpp-near --config /path/to/config.toml ...`

### Environment Variables

```bash
# Intents API key
export OUTLAYER_API_KEY=wk_...

# NEAR account (standard)
export NEAR_ACCOUNT_ID=kampouse.near
export NEAR_PRIVATE_KEY=ed25519:...

# Quiet mode (suppress info messages)
export MPP_NEAR_QUIET=1
```

### Full Config Example

```toml
# Default payment method
method = "intents"

# Standard provider configuration
[standard]
account = "kampouse.near"
private_key = "ed25519:5Kd3NBUAdq5WJn5h1ZaF5GKK8cFQRkXdE6..."
rpc_url = "https://rpc.mainnet.near.org"

# Intents provider configuration
[intents]
api_key = "wk_your_api_key_here"
api_url = "https://api.outlayer.fastnear.com"
```

## API Reference

### IntentsProvider (Gasless)

```rust
impl IntentsProvider {
    // Create provider
    pub fn new(api_key: String) -> Self;
    
    // Get wallet account ID
    pub async fn get_account_id(&self) -> Result<String>;
    
    // Check NEAR balance
    pub async fn check_balance(&self) -> Result<NearAmount>;
    
    // Check token balance (intents)
    pub async fn check_intents_balance(&self, token: &str) -> Result<NearAmount>;
    
    // Transfer NEAR (gasless)
    pub async fn transfer(&self, recipient: &AccountId, amount: NearAmount) -> Result<TransactionHash>;
    
    // Transfer token (gasless)
    pub async fn transfer_token(&self, token: &str, recipient: &AccountId, amount: NearAmount) -> Result<TransactionHash>;
    
    // Swap tokens (gasless)
    pub async fn swap(&self, token_in: &str, token_out: &str, amount: NearAmount, min_out: Option<NearAmount>) -> Result<SwapResult>;
    
    // Create payment check (agent-to-agent)
    pub async fn create_payment_check(&self, token: &str, amount: NearAmount, memo: Option<&str>, expires_in: Option<u64>) -> Result<PaymentCheck>;
    
    // Claim payment check
    pub async fn claim_payment_check(&self, check_key: &str, amount: Option<NearAmount>) -> Result<NearAmount>;
    
    // List available tokens
    pub async fn list_tokens(&self) -> Result<Vec<TokenInfo>>;
}
```

### NearProvider (Standard)

```rust
impl NearProvider {
    // Create provider
    pub fn new(account_id: AccountId, private_key: String, rpc_url: &str) -> Result<Self>;
    
    // Check account balance
    pub async fn check_balance(&self) -> Result<NearAmount>;
    
    // Transfer NEAR
    pub async fn transfer(&self, recipient: &AccountId, amount: NearAmount) -> Result<TransactionHash>;
    
    // Transfer NEP-141 token
    pub async fn transfer_token(&self, token_contract: &AccountId, recipient: &AccountId, amount: NearAmount) -> Result<TransactionHash>;
}
```

## Supported Tokens

### NEAR
- NEAR (native)

### NEP-141 Tokens
- USDC: `17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1`
- USDT: `usdt.tether-token.near`

### Cross-Chain (Intents only)
- Ethereum, Bitcoin, Solana, Arbitrum, Base, Polygon, Optimism, Avalanche, BSC, TON, Aptos, Sui, and more

Use `mpp-near tokens --api-key wk_...` to see full list.

## Error Handling

### Common Errors

**"API key required"**
- Set `--api-key` or configure in config file
- Get key from https://outlayer.fastnear.com

**"Account required"**
- Set `--account` for standard method
- Or use `--method intents`

**"Private key required"**
- Set `--private-key` for standard method
- Format: `ed25519:...`

**"Insufficient balance"**
- Check balance with `mpp-near balance`
- For standard: need NEAR for gas
- For intents: need tokens in custody wallet

**"Invalid recipient"**
- Must be valid NEAR account ID
- Format: `account.near` or `sub.account.near`

## Troubleshooting

### Verbose Output

```bash
mpp-near pay --recipient test.near --amount 1 --verbose
```

### Test Configuration

```bash
mpp-near config
```

### Check Balance First

```bash
# Always check balance before paying
mpp-near balance --api-key wk_...
```

## Security

### API Keys
- Store securely (environment variables or config file)
- Never commit to git
- Rotate if compromised

### Private Keys
- **Standard method only** - Intents doesn't need your private key
- Store securely
- Never share or commit

### Intents Custody
- Keys stored in TEE (Trusted Execution Environment)
- OutLayer manages security
- Trade-off: gasless but not "your keys, your crypto"

## Architecture

```
mpp-near
├── src/
│   ├── lib.rs           # Library entry
│   ├── client/
│   │   ├── provider.rs  # Standard NEAR payments
│   │   ├── intents.rs   # Gasless Intents payments
│   │   ├── middleware.rs # HTTP 402 handler
│   │   └── signer.rs    # ED25519 signatures
│   ├── server/
│   │   ├── verifier.rs  # Payment verification
│   │   └── extractor.rs # Axum extractors
│   ├── types/
│   │   └── mod.rs       # AccountId, NearAmount, etc.
│   └── bin/
│       └── mpp-near.rs  # CLI binary
└── examples/
    ├── near_client.rs   # Standard client example
    ├── near_server.rs   # Payment server example
    └── intents_client.rs # Gasless client example
```

## Protocol Compliance

mpp-near implements:

- **IETF MPP** - Machine Payments Protocol (paymentauth.org)
- **HTTP 402** - Payment Required status code
- **JSON-RPC transport** - For MCP/AI agents
- **NEAR Intents** - Gasless payment protocol

## Resources

- **GitHub:** https://github.com/Kampouse/mpp-near
- **MPP Spec:** https://paymentauth.org
- **NEAR Intents:** https://outlayer.fastnear.com
- **NEAR RPC:** https://rpc.mainnet.near.org

## License

MIT OR Apache-2.0

## Credits

- Forked from [tempoxyz/mpp-rs](https://github.com/tempoxyz/mpp-rs)
- NEAR Intents by OutLayer
- MPP by Tempo Labs
