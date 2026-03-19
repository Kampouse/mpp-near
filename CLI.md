# mpp-near CLI - Full Command Reference

NEAR payment CLI for Machine Payments Protocol (MPP). Complete implementation with 9 commands.

## Installation

```bash
cd ~/.openclaw/workspace/mpp-near
cargo install --path .

# Now available globally
mpp-near --help
```

## Commands Overview

| Command | Standard | Intents | Description |
|---------|----------|---------|-------------|
| `pay` | ✅ | ✅ | Send payment |
| `balance` | ✅ | ✅ | Check balance |
| `verify` | ✅ | ✅ | Verify transaction |
| `server` | ✅ | ✅ | Start payment server |
| `tokens` | ❌ | ✅ | List available tokens |
| `create-check` | ❌ | ✅ | Create payment check |
| `claim-check` | ❌ | ✅ | Claim payment check |
| `swap` | ❌ | ✅ | Swap tokens |
| `config` | ✅ | ✅ | Show configuration |

## Command Details

### 1. `pay` - Send Payment

Send NEAR or tokens to a recipient.

```bash
# Gasless payment (Intents)
mpp-near pay --recipient merchant.near --amount 1 --api-key wk_...

# Standard payment (requires gas)
mpp-near pay \
  --recipient merchant.near \
  --amount 1 \
  --method standard \
  --account kampouse.near \
  --private-key ed25519:...

# Send USDC (gasless)
mpp-near pay \
  --recipient merchant.near \
  --amount 10 \
  --token usdc \
  --api-key wk_...

# With memo
mpp-near pay \
  --recipient merchant.near \
  --amount 1 \
  --memo "Invoice #123" \
  --api-key wk_...
```

**Options:**
- `--recipient, -r` - Account ID to receive payment (required)
- `--amount, -a` - Amount in NEAR (e.g., "1.5") (required)
- `--token, -t` - Token: near, usdc, usdt (default: near)
- `--memo, -m` - Optional memo
- `--method, -m` - Payment method: standard or intents (default: intents)

### 2. `balance` - Check Balance

Check account balance.

```bash
# Intents wallet balance (gasless)
mpp-near balance --api-key wk_...

# Standard account balance
mpp-near balance \
  --account kampouse.near \
  --method standard
```

**Output:**
```
✓ Balance retrieved
  Account: wallet_abc123.near
  Balance: 10.500000 NEAR
  USDC:    150.250000
```

### 3. `verify` - Verify Transaction

Verify a transaction on-chain.

```bash
# Basic verification
mpp-near verify --tx-hash 0x123abc...

# With expected values
mpp-near verify \
  --tx-hash 0x123abc... \
  --expected-amount 1.5 \
  --expected-recipient merchant.near
```

**Output:**
```
Transaction Details:
  Hash:                0x123abc...
  Expected amount:     1.5 NEAR
  Expected recipient:  merchant.near
```

### 4. `server` - Start Payment Server

Start HTTP server that accepts MPP payments.

```bash
# Basic server
mpp-near server \
  --recipient merchant.near \
  --port 3000

# With minimum amount
mpp-near server \
  --recipient merchant.near \
  --port 3000 \
  --min-amount 0.1
```

**Output:**
```
✓ Server listening on http://0.0.0.0:3000

Endpoints:
  GET /          - API info
  GET /health    - Health check
  GET /challenge - Create payment challenge

Ready to accept payments via MPP!
```

**Endpoints:**

```bash
# Check health
curl http://localhost:3000/health
# {"status": "healthy"}

# Get payment challenge
curl http://localhost:3000/challenge
# {"status": "payment_required", "challenge": {...}}
```

### 5. `tokens` - List Available Tokens

List all tokens available for Intents (20+ chains).

```bash
mpp-near tokens --api-key wk_...
```

**Output:**
```
✓ Found 156 tokens

NEAR
  near   - NEAR Protocol (24 decimals)
  ... and 0 more

ETHEREUM
  eth    - Ethereum (18 decimals)
  usdc   - USD Coin (6 decimals)
  usdt   - Tether USD (6 decimals)
  ... and 7 more

BITCOIN
  btc    - Bitcoin (8 decimals)
  ... and 2 more

SOLANA
  sol    - Solana (9 decimals)
  ... and 5 more
```

### 6. `create-check` - Create Payment Check

Create a payment check for agent-to-agent transfers.

```bash
# Create 10 USDC check
mpp-near create-check \
  --amount 10 \
  --token usdc \
  --memo "Payment for API access" \
  --expires-in 86400 \
  --api-key wk_...

# Create 1 NEAR check (24h expiry)
mpp-near create-check \
  --amount 1 \
  --token near \
  --api-key wk_...
```

**Output:**
```
✓ Payment check created

  Check ID:  abc123...
  Check Key: xyz789...
  Amount:    10 USDC
  Memo:      Payment for API access
  Expires:   2026-03-19T12:00:00Z

Share the check key with the recipient to claim.
```

**Options:**
- `--amount, -a` - Amount in token units (required)
- `--token, -t` - Token: near, usdc, usdt (default: near)
- `--memo, -m` - Optional memo
- `--expires-in, -e` - Expiry in seconds (default: 86400 = 24h)

### 7. `claim-check` - Claim Payment Check

Claim a payment check.

```bash
# Claim full amount
mpp-near claim-check \
  --check-key xyz789... \
  --api-key wk_...

# Claim partial amount
mpp-near claim-check \
  --check-key xyz789... \
  --amount 5 \
  --api-key wk_...
```

**Output:**
```
✓ Payment check claimed!
  Amount claimed: 10 USDC
```

### 8. `swap` - Swap Tokens

Swap one token for another (gasless).

```bash
# Swap 1 NEAR to USDC
mpp-near swap \
  --from near \
  --to usdc \
  --amount 1 \
  --api-key wk_...

# Swap 100 USDC to NEAR
mpp-near swap \
  --from usdc \
  --to near \
  --amount 100 \
  --api-key wk_...

# Swap ETH to USDC (cross-chain)
mpp-near swap \
  --from eth \
  --to usdc \
  --amount 0.5 \
  --api-key wk_...
```

**Output:**
```
✓ Swap completed (gasless)!
  Request ID: req_abc123...
  Amount out: 2.500000 USDC
  Intent:     0xdef456...
```

**Options:**
- `--from` - Token to swap from (required)
- `--to` - Token to swap to (required)
- `--amount, -a` - Amount to swap (required)

### 9. `config` - Show Configuration

Display current configuration and examples.

```bash
mpp-near config
```

**Output:**
```
Configuration:

Method:     intents
Config:     None
RPC URL:    None
Account:    None
API Key:    (set)

Commands:
  pay          - Send a payment
  balance      - Check account balance
  verify       - Verify a transaction
  server       - Start payment server
  tokens       - List available tokens
  create-check - Create payment check
  claim-check  - Claim payment check
  swap         - Swap tokens
  config       - Show this configuration

Example usage:
  mpp-near pay --recipient merchant.near --amount 1
  mpp-near pay --recipient merchant.near --amount 10 --token usdc
  mpp-near balance
  mpp-near tokens
  mpp-near swap --from near --to usdc --amount 1
  mpp-near create-check --amount 10 --token usdc
```

## Global Options

These options work with all commands:

```bash
--method <standard|intents>    Payment method (default: intents)
--config <PATH>                Config file path (default: ~/.mpp-near/config.toml)
--rpc-url <URL>                RPC URL for standard provider
--account <ID>                 Account ID for standard provider
--private-key <KEY>            Private key for standard provider (ed25519:...)
--api-key <KEY>                API key for intents provider (wk_...)
--verbose, -v                  Verbose output
--help, -h                     Show help
--version, -V                  Show version
```

## Configuration File

Create `~/.mpp-near/config.toml`:

```toml
# Default payment method
method = "intents"

# Standard provider configuration
[standard]
account = "kampouse.near"
private_key = "ed25519:5Kd3..."
rpc_url = "https://rpc.mainnet.near.org"

# Intents provider configuration
[intents]
api_key = "wk_your_api_key_here"
api_url = "https://api.outlayer.fastnear.com"
```

Then commands become simpler:

```bash
# Uses config file
mpp-near pay --recipient merchant.near --amount 1

# Override config
mpp-near pay --recipient merchant.near --amount 1 --method standard
```

## Environment Variables

```bash
# Intents API key
export OUTLAYER_API_KEY=wk_...

# NEAR account (standard)
export NEAR_ACCOUNT_ID=kampouse.near
export NEAR_PRIVATE_KEY=ed25519:...

# Quiet mode (suppress info messages)
export MPP_NEAR_QUIET=1
```

## Examples

### Pay for API Access

```bash
#!/bin/bash
# pay-for-api.sh

API_KEY="wk_..."
PROVIDER="api-provider.near"

mpp-near pay \
  --recipient $PROVIDER \
  --amount 0.1 \
  --token near \
  --memo "API access - $(date)" \
  --api-key $API_KEY
```

### Automated Swaps

```bash
#!/bin/bash
# swap-to-usdc.sh

# Swap NEAR to USDC when balance > 10
BALANCE=$(mpp-near balance --api-key $API_KEY | grep "Balance:" | awk '{print $2}')

if (( $(echo "$BALANCE > 10" | bc -l) )); then
  mpp-near swap \
    --from near \
    --to usdc \
    --amount 5 \
    --api-key $API_KEY
fi
```

### Payment Check for Agents

```bash
#!/bin/bash
# create-agent-payment.sh

# Create payment check
CHECK=$(mpp-near create-check \
  --amount 5 \
  --token usdc \
  --memo "Agent payment" \
  --api-key $API_KEY)

CHECK_KEY=$(echo "$CHECK" | grep "Check Key:" | awk '{print $3}')

# Send to agent
echo "Agent can claim with: mpp-near claim-check --check-key $CHECK_KEY"
```

### Start Paid API Server

```bash
#!/bin/bash
# start-paid-api.sh

mpp-near server \
  --recipient merchant.near \
  --port 3000 \
  --min-amount 0.1

# Clients can now access via HTTP 402
# curl http://localhost:3000/api/expensive
# HTTP/1.1 402 Payment Required
```

## Error Handling

### Common Errors

**"API key required"**
```bash
# Solution: Set API key
mpp-near pay --recipient test.near --amount 1 --api-key wk_...
```

**"Insufficient balance"**
```bash
# Solution: Check balance first
mpp-near balance --api-key wk_...
```

**"Invalid recipient"**
```bash
# Solution: Use valid NEAR account ID
mpp-near pay --recipient "invalid..near" --amount 1  # ❌ Error
mpp-near pay --recipient "valid.near" --amount 1     # ✅ Works
```

## Verbose Mode

Get detailed output:

```bash
mpp-near pay --recipient test.near --amount 1 --verbose --api-key wk_...
```

## Exit Codes

- `0` - Success
- `1` - Error (invalid args, payment failed, etc.)
- `2` - Configuration error
- `3` - Network error

## Binary Info

- **Size:** ~4.5MB (release build)
- **Compile time:** ~3 seconds
- **Dependencies:** Minimal (only what's needed)

## License

MIT OR Apache-2.0

## Links

- **GitHub:** https://github.com/Kampouse/mpp-near
- **MPP Spec:** https://paymentauth.org
- **Intents:** https://outlayer.fastnear.com
