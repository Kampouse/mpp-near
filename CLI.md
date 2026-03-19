# mpp-near CLI

CLI tool for NEAR payments via Machine Payments Protocol (MPP).

## Installation

```bash
cargo install --path .
```

## Quick Start

### 1. Configure

Create `~/.mpp-near/config.toml`:

```toml
method = "intents"

[standard]
account = "kampouse.near"
private_key = "ed25519:..."
rpc_url = "https://rpc.mainnet.near.org"

[intents]
api_key = "wk_..."
```

### 2. Send Payment

```bash
# Gasless payment via Intents
mpp-near pay --recipient merchant.near --amount 1

# Standard payment (requires gas)
mpp-near pay --recipient merchant.near --amount 1 --method standard
```

## Commands

### `pay` - Send payment

```bash
# Send 1 NEAR (gasless)
mpp-near pay --recipient merchant.near --amount 1

# Send 10 USDC (gasless)
mpp-near pay --recipient merchant.near --amount 10 --token usdc

# Send with memo
mpp-near pay --recipient merchant.near --amount 1 --memo "Invoice #123"

# Standard payment (requires NEAR for gas)
mpp-near pay --recipient merchant.near --amount 1 --method standard
```

### `balance` - Check balance

```bash
# Check your balance
mpp-near balance

# Check specific account (standard only)
mpp-near balance --account kampouse.near --method standard
```

### `tokens` - List available tokens (intents)

```bash
mpp-near tokens
```

### `create-check` - Create payment check (intents)

```bash
# Create a 10 USDC check
mpp-near create-check --amount 10 --token usdc --memo "Payment for services"

# Output:
#   Check ID:  abc123...
#   Check Key: xyz789...
#   Amount:    10 USDC
```

### `claim-check` - Claim payment check (intents)

```bash
# Claim full amount
mpp-near claim-check --check-key xyz789...

# Claim partial amount
mpp-near claim-check --check-key xyz789... --amount 5
```

### `swap` - Swap tokens (intents)

```bash
# Swap 1 NEAR to USDC
mpp-near swap --from near --to usdc --amount 1

# Swap 100 USDC to NEAR
mpp-near swap --from usdc --to near --amount 100
```

### `server` - Start payment server

```bash
# Start server on port 3000
mpp-near server --port 3000 --recipient merchant.near --min-amount 0.001

# Output:
#   ✓ Server listening on http://0.0.0.0:3000
#
#   Endpoints:
#     GET /          - API info
#     GET /health    - Health check
#     GET /challenge - Create payment challenge
```

### `config` - Show configuration

```bash
mpp-near config
```

## Options

### Global Options

```bash
--method <standard|intents>    Payment method (default: intents)
--config <PATH>                Config file path (default: ~/.mpp-near/config.toml)
--verbose, -v                  Verbose output
```

### Standard Provider Options

```bash
--account <ID>                 NEAR account ID
--private-key <KEY>            Private key (ed25519:...)
--rpc-url <URL>                RPC endpoint (default: https://rpc.mainnet.near.org)
```

### Intents Provider Options

```bash
--api-key <KEY>                OutLayer API key (wk_...)
```

## Payment Methods

### Standard (requires gas)

- ✅ Works with any NEAR wallet
- ✅ Direct on-chain transactions
- ❌ Requires NEAR for gas fees
- ❌ Slower (~1-2 seconds)

**When to use:** You have NEAR for gas, want full control

### Intents (gasless)

- ✅ No gas required
- ✅ Instant (solver pays gas)
- ✅ Cross-chain swaps (20+ chains)
- ✅ Agent-to-agent payment checks
- ❌ Requires OutLayer API key
- ❌ Custody wallet (not your keys)

**When to use:** No NEAR for gas, want gasless, need cross-chain

## Examples

### Send payment from script

```bash
#!/bin/bash
# Pay for API access
mpp-near pay \
  --recipient api-provider.near \
  --amount 0.1 \
  --token near \
  --method intents \
  --api-key $OUTLAYER_API_KEY
```

### Check balance before paying

```bash
# Check if you have enough
balance=$(mpp-near balance --method intents | grep "NEAR" | awk '{print $2}')
if (( $(echo "$balance >= 1.0" | bc -l) )); then
  mpp-near pay --recipient merchant.near --amount 1
else
  echo "Insufficient balance"
fi
```

### Create payment link

```bash
# Create check
check=$(mpp-near create-check --amount 5 --token usdc --memo "Payment for API")
check_key=$(echo "$check" | grep "Check Key:" | awk '{print $3}')

# Send to recipient
echo "Claim your payment: mpp-near claim-check --check-key $check_key"
```

### Start paid API server

```bash
# Start server requiring 0.1 NEAR per request
mpp-near server \
  --port 3000 \
  --recipient merchant.near \
  --min-amount 0.1

# Clients can now pay via HTTP 402
curl -i http://localhost:3000/api/expensive-endpoint
# HTTP/1.1 402 Payment Required
# WWW-Authenticate: Payment ...
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

## Exit Codes

- `0` - Success
- `1` - Error (invalid args, payment failed, etc.)
- `2` - Configuration error
- `3` - Network error

## License

MIT OR Apache-2.0
