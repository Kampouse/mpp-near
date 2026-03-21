# MPP-NEAR Examples Testing Guide

This guide provides comprehensive instructions for running and testing the MPP-NEAR examples end-to-end.

## Overview

The MPP-NEAR project includes three comprehensive examples that demonstrate the complete payment flow:

1. **full_server.rs** - A complete MPP server supporting both standard NEAR and NEAR Intents payments
2. **full_client.rs** - A complete client that can pay for services using both methods
3. **end_to_end_test.rs** - A comprehensive test suite covering the entire payment flow

## Prerequisites

### Required

- Rust 1.70 or later
- Cargo (comes with Rust)
- Git

### Optional (for real payments)

- NEAR account on mainnet or testnet
- NEAR private key
- OutLayer API key (for NEAR Intents)
- Testnet NEAR tokens (for testing without real money)

### Installation

```bash
# Clone the repository
git clone https://github.com/Kampouse/mpp-near.git
cd mpp-near

# Build the project
cargo build --release --all-features

# Run tests to verify installation
cargo test --all-features
```

## Quick Start

### 1. Run the Server

```bash
# Start the server with all features
cargo run --example full_server --features server

# The server will start on http://localhost:3000
```

Expected output:
```
╔════════════════════════════════════════════════════════════╗
║          MPP-NEAR Full Server Example                     ║
╠════════════════════════════════════════════════════════════╣
║  Recipient: merchant.near                                    ║
║  Challenge TTL: 300                                         ║
║  Payment Methods: near, near-intents                    ║
╠════════════════════════════════════════════════════════════╣
║  Endpoints:                                             ║
║  /health                FREE       0 USD               ║
║  /api/v1/ping           PAID       0.0001 USDC         ║
║  /api/v1/analyze        PAID       0.001 USDC          ║
║  /api/v1/generate       PAID       0.01 USDC           ║
║  /api/v1/complex        PAID       0.1 USDC            ║
╚════════════════════════════════════════════════════════════╝

🚀 Server listening on http://0.0.0.0:3000
```

### 2. Run the Client

In a new terminal:

```bash
# Run the client with all features
cargo run --example full_client --features client,intents
```

Expected output:
```
╔════════════════════════════════════════════════════════════╗
║          MPP-NEAR Full Client Example                     ║
╚════════════════════════════════════════════════════════════╝

Configuration:
  Server URL: http://localhost:3000
  Payment Method: auto
  Max Retries: 3
  Account ID: None
  Intents API Key: None

✓ MPP Client created

┌────────────────────────────────────────────────────────────┐
│ Example 1: Health Check (Free)                              │
└────────────────────────────────────────────────────────────┘
✓ Status: 200 OK
✓ Body: {"status":"ok","message":"Health check passed","paid":false}
```

### 3. Run End-to-End Tests

```bash
# Run the comprehensive test suite
cargo run --example end_to_end_test --features client,server,intents
```

Expected output:
```
╔════════════════════════════════════════════════════════════╗
║          MPP-NEAR End-to-End Test Suite                   ║
╚════════════════════════════════════════════════════════════╝

╔════════════════════════════════════════════════════════════╗
║  Core Payment Flow Tests                                    ║
╚════════════════════════════════════════════════════════════╝
  ✓ PASS (2 ms) - Complete Payment Flow
  ✓ PASS (1 ms) - Challenge Binding Verification
  ✓ PASS (0 ms) - Credential Serialization Round-trip
  ✓ PASS (0 ms) - Challenge Serialization Round-trip
  ✓ PASS (0 ms) - Challenge Expiration
  ✓ PASS (0 ms) - Receipt Serialization Round-trip
  ✓ PASS (0 ms) - Problem Error Handling

╔════════════════════════════════════════════════════════════╗
║                   Test Summary                               ║
╠════════════════════════════════════════════════════════════╣
║  Total Tests: 7                                             ║
║  Passed:      7                                             ║
║  Failed:      0                                             ║
║  Duration:    3 ms                                          ║
╚════════════════════════════════════════════════════════════╝

✅ All tests passed!
```

## Detailed Examples

### Example 1: Free Endpoint (No Payment Required)

```bash
curl http://localhost:3000/health
```

Response:
```json
{
  "status": "healthy",
  "service": "mpp-near-server",
  "version": "1.0.0",
  "timestamp": 1710952800
}
```

### Example 2: Paid Endpoint Without Payment (Gets Challenge)

```bash
curl -v http://localhost:3000/api/v1/ping
```

Response (402 Payment Required):
```
< HTTP/1.1 402 Payment Required
< www-authenticate: Payment realm="api.example.com", method="near-intents", intent="charge", request="eyJhbW91bnQiOiIwLjAwMDEiLCJyZWNpcGllbnQiOiJtZXJjaGFudC5uZWFyIn0=", id="abc123...", expires="2025-01-15T12:05:00Z", description="Simple ping"

{
  "status": "payment_required",
  "challenge": {
    "id": "abc123...",
    "realm": "api.example.com",
    "method": "near-intents",
    "intent": "charge",
    "amount": "0.0001",
    "currency": "USDC",
    "description": "Simple ping",
    "expires": "2025-01-15T12:05:00Z"
  }
}
```

### Example 3: Paid Endpoint With Payment

```bash
# First, get the challenge
CHALLENGE_RESPONSE=$(curl -s http://localhost:3000/api/v1/ping)

# Extract the challenge ID
CHALLENGE_ID=$(echo $CHALLENGE_RESPONSE | jq -r '.challenge.id')

# Extract the WWW-Authenticate header
WWW_AUTHENTICATE=$(echo $CHALLENGE_RESPONSE | jq -r '.www_authenticate')

# Create a mock credential
AUTHORIZATION=$(cat <<EOF | base64url
{
  "challenge": {
    "id": "$CHALLENGE_ID",
    "realm": "api.example.com",
    "method": "near-intents",
    "intent": "charge",
    "request": "eyJhbW91bnQiOiIwLjAwMDEiLCJyZWNpcGllbnQiOiJtZXJjaGFudC5uZWFyIn0="
  },
  "payload": {
    "proof": "test_payment_$CHALLENGE_ID"
  }
}
EOF
)

# Make request with payment
curl -H "Authorization: Payment $AUTHORIZATION" http://localhost:3000/api/v1/ping
```

Response (200 OK):
```
< HTTP/1.1 200 OK
< payment-receipt: Payment {"id":"receipt-abc...", "status":"confirmed", "amount":"0.0001", "token":"USDC"}

{
  "status": "pong",
  "timestamp": "2025-03-20T12:05:00Z",
  "message": "Ping successful"
}
```

### Example 4: Get Pricing Information

```bash
curl http://localhost:3000/pricing
```

Response:
```json
{
  "currency": "USDC",
  "endpoints": {
    "/health": {
      "amount": "0",
      "currency": "USD",
      "description": "Health check (free)"
    },
    "/api/v1/ping": {
      "amount": "0.0001",
      "currency": "USDC",
      "description": "Simple ping"
    },
    "/api/v1/analyze": {
      "amount": "0.001",
      "currency": "USDC",
      "description": "Data analysis"
    },
    "/api/v1/generate": {
      "amount": "0.01",
      "currency": "USDC",
      "description": "Content generation"
    },
    "/api/v1/complex": {
      "amount": "0.1",
      "currency": "USDC",
      "description": "Complex computation"
    }
  }
}
```

## Configuration

### Server Configuration

Set these environment variables before running the server:

```bash
# Required: HMAC secret for challenge binding
export MPP_HMAC_SECRET="your-secret-here-change-in-production"

# Required: Recipient account for payments
export MPP_RECIPIENT="merchant.near"

# Optional: NEAR RPC URL (default: https://rpc.mainnet.near.org)
export MPP_RPC_URL="https://rpc.testnet.near.org"

# Optional: Challenge TTL in seconds (default: 300)
export MPP_CHALLENGE_TTL="300"

# Optional: OutLayer API key for intents (for production)
export OUTLAYER_API_KEY="your-api-key-here"
```

### Client Configuration

Set these environment variables before running the client:

```bash
# Required: Server URL
export MPP_SERVER_URL="http://localhost:3000"

# Optional: Payment method: "near", "near-intents", or "auto" (default: auto)
export MPP_PAYMENT_METHOD="auto"

# Optional: Maximum retry attempts (default: 3)
export MPP_MAX_RETRIES="3"

# Optional: Retry delay in seconds (default: 2)
export MPP_RETRY_DELAY="2"

# For standard NEAR payments:
export NEAR_ACCOUNT_ID="your-account.near"
export NEAR_PRIVATE_KEY="ed25519:..."
export NEAR_RPC_URL="https://rpc.testnet.near.org"

# For NEAR Intents payments:
export OUTLAYER_API_KEY="your-api-key"
export OUTLAYER_API_URL="https://outlayer.fastnear.com"
```

## Testing with Real Payments

### Option 1: Testnet (Recommended for Testing)

1. **Create a Testnet Account**
   ```bash
   # Create account at https://wallet.testnet.near.org/
   # Or use NEAR CLI
   near create-account account.testnet --useFaucet
   ```

2. **Get Testnet NEAR**
   ```bash
   # Use faucet
   curl https://wallet.testnet.near.org/account/faucet
   
   # Or transfer from mainnet
   near send your-account.testnet --amount 10
   ```

3. **Configure for Testnet**
   ```bash
   export NEAR_RPC_URL="https://rpc.testnet.near.org"
   export MPP_RECIPIENT="your-account.testnet"
   export NEAR_ACCOUNT_ID="your-account.testnet"
   export NEAR_PRIVATE_KEY="your-testnet-private-key"
   ```

4. **Run Examples**
   ```bash
   cargo run --example full_server --features server
   cargo run --example full_client --features client
   ```

### Option 2: Mainnet (Real Payments)

⚠️ **Warning:** Use mainnet only with real money. Start with small amounts!

1. **Create a Mainnet Account**
   ```bash
   near create-account your-account.near --masterAccount your-account --initialBalance 10
   ```

2. **Configure for Mainnet**
   ```bash
   export NEAR_RPC_URL="https://rpc.mainnet.near.org"
   export MPP_RECIPIENT="merchant.near"
   export NEAR_ACCOUNT_ID="your-account.near"
   export NEAR_PRIVATE_KEY="your-mainnet-private-key"
   ```

3. **Run Examples**
   ```bash
   cargo run --example full_server --features server
   cargo run --example full_client --features client
   ```

### Option 3: NEAR Intents (Gasless)

1. **Get OutLayer API Key**
   - Visit https://outlayer.fastnear.com
   - Sign up and get your API key
   - Fund your gasless wallet with USDC

2. **Configure for Intents**
   ```bash
   export OUTLAYER_API_KEY="wk_your-api-key"
   export OUTLAYER_API_URL="https://outlayer.fastnear.com"
   ```

3. **Run Examples**
   ```bash
   cargo run --example full_server --features server
   cargo run --example full_client --features client,intents
   ```

## Testing Scenarios

### Scenario 1: End-to-End Flow with Mock Payments

```bash
# Terminal 1: Start server
cargo run --example full_server --features server

# Terminal 2: Run client with mock payments
cargo run --example full_client --features client

# Expected: All requests succeed with mock payments
```

### Scenario 2: End-to-End Flow with Real NEAR Payments

```bash
# Terminal 1: Start server
export MPP_RECIPIENT="your-account.testnet"
export MPP_RPC_URL="https://rpc.testnet.near.org"
cargo run --example full_server --features server

# Terminal 2: Run client with real NEAR payments
export NEAR_ACCOUNT_ID="your-account.testnet"
export NEAR_PRIVATE_KEY="your-testnet-private-key"
export NEAR_RPC_URL="https://rpc.testnet.near.org"
export MPP_PAYMENT_METHOD="near"
cargo run --example full_client --features client

# Expected: Real NEAR transfers executed
```

### Scenario 3: End-to-End Flow with NEAR Intents

```bash
# Terminal 1: Start server
export OUTLAYER_API_KEY="your-api-key"
cargo run --example full_server --features server

# Terminal 2: Run client with NEAR Intents
export OUTLAYER_API_KEY="your-api-key"
export MPP_PAYMENT_METHOD="near-intents"
cargo run --example full_client --features client,intents

# Expected: Gasless payments via OutLayer
```

### Scenario 4: Comprehensive Test Suite

```bash
# Run all tests
cargo run --example end_to_end_test --features client,server,intents

# Expected: All 20+ tests pass
```

## Troubleshooting

### Issue: Server won't start

**Error:** `Failed to bind to address`

**Solution:**
```bash
# Check if port 3000 is in use
lsof -i :3000

# Kill existing process or use different port
# Modify the server code to use port 3001
```

### Issue: Client connection refused

**Error:** `Failed to fetch pricing: Connection refused`

**Solution:**
```bash
# Make sure server is running
curl http://localhost:3000/health

# Check server is listening on correct port
netstat -an | grep 3000
```

### Issue: Invalid Account ID

**Error:** `InvalidAccountId("invalid-account")`

**Solution:**
```bash
# Ensure account ID follows NEAR rules:
# - Contains only lowercase letters, numbers, underscore, or hyphen
# - Each part separated by dots (.)
# - Each part 1-63 characters
# - Total length <= 64 characters

# Examples:
# account.near ✓
# sub.account.near ✓
# account@test.near ✗ (invalid character)
# very-long-name-that-is-too-long.near ✗ (too long)
```

### Issue: Challenge expired

**Error:** `Challenge expired`

**Solution:**
```bash
# Increase challenge TTL
export MPP_CHALLENGE_TTL="600"  # 10 minutes instead of 5

# Or complete payment faster
```

### Issue: Payment verification failed

**Error:** `Payment verification failed`

**Solution:**
```bash
# Check that proof matches transaction
# Ensure recipient account is correct
# Verify amount matches challenge requirements
# Check that transaction has enough confirmations
```

### Issue: Out of gas

**Error:** `TransactionFailed: Out of gas`

**Solution:**
```bash
# Use NEAR Intents for gasless payments
# Or ensure account has enough NEAR for gas
# Increase gas limit in config
```

### Issue: Storage registration required

**Error:** `Storage registration required`

**Solution:**
```bash
# Register storage for tokens
# Use the CLI to register storage:
cargo run --bin mpp-near -- storage-deposit --token usdt.tether-token.near

# Or use the API server
curl -X POST http://localhost:3456/storage-deposit \
  -H "Content-Type: application/json" \
  -d '{"token":"usdt.tether-token.near"}'
```

## Advanced Testing

### Load Testing

```bash
# Install bombardier
go install github.com/codesenberg/bombardier@latest

# Test free endpoint
bombardier -c 10 -n 1000 http://localhost:3000/health

# Test paid endpoint with mock payments (modify server to accept all)
bombardier -c 5 -n 100 http://localhost:3000/api/v1/ping
```

### Integration with CI/CD

```yaml
# .github/workflows/test.yml
name: Test MPP-NEAR
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run tests
        run: |
          cargo test --all-features
          cargo run --example end_to_end_test --features client,server,intents
```

### Manual Payment Flow Testing

```bash
# Step 1: Create challenge
CHALLENGE=$(curl -s http://localhost:3000/api/v1/ping)
echo $CHALLENGE | jq

# Step 2: Extract challenge details
CHALLENGE_ID=$(echo $CHALLENGE | jq -r '.challenge.id')
REALM=$(echo $CHALLENGE | jq -r '.challenge.realm')
METHOD=$(echo $CHALLENGE | jq -r '.challenge.method')
INTENT=$(echo $CHALLENGE | jq -r '.challenge.intent')
REQUEST=$(echo $CHALLENGE | jq -r '.challenge.request')

# Step 3: Make payment (using CLI)
PAYMENT=$(cargo run --bin mpp-near -- pay \
  --to merchant.near \
  --amount 0.0001 \
  --token USDC)

# Step 4: Extract transaction hash
TX_HASH=$(echo $PAYMENT | jq -r '.tx_hash')

# Step 5: Create credential
CREDENTIAL=$(cat <<EOF
{
  "challenge": {
    "id": "$CHALLENGE_ID",
    "realm": "$REALM",
    "method": "$METHOD",
    "intent": "$INTENT",
    "request": "$REQUEST"
  },
  "payload": {
    "proof": "$TX_HASH"
  }
}
EOF
)

# Step 6: Encode credential
AUTH=$(echo "$CREDENTIAL" | base64 -w 0)

# Step 7: Make request with payment
curl -H "Authorization: Payment $AUTH" \
  http://localhost:3000/api/v1/ping
```

## Best Practices

### For Development

1. **Always use testnet for development**
   ```bash
   export NEAR_RPC_URL="https://rpc.testnet.near.org"
   ```

2. **Use mock payments in local development**
   ```bash
   # Server will accept mock payments
   # Look for `.with_mocks()` in the code
   ```

3. **Enable debug logging**
   ```bash
   export RUST_LOG=debug
   ```

### For Testing

1. **Test edge cases**
   - Expired challenges
   - Invalid credentials
   - Wrong amounts
   - Invalid accounts

2. **Test both payment methods**
   - Standard NEAR
   - NEAR Intents

3. **Verify spec compliance**
   - Run the end-to-end test suite
   - Check RFC 9457 Problem details
   - Verify header formats

### For Production

1. **Use environment variables for secrets**
   ```bash
   export MPP_HMAC_SECRET=$(openssl rand -hex 32)
   ```

2. **Use HTTPS**
   - Configure TLS certificates
   - Use secure RPC endpoints

3. **Monitor payments**
   - Log all transactions
   - Set up alerts for failures
   - Track revenue

4. **Implement proper error handling**
   - Retry failed payments
   - Handle rate limits
   - Provide clear error messages

## Next Steps

After successfully testing the examples:

1. **Explore the Codebase**
   - Read the source code
   - Understand the MPP protocol
   - Learn about NEAR Intents

2. **Build Your Own Service**
   - Modify the server example
   - Add your own endpoints
   - Implement custom pricing

3. **Integrate with Your Application**
   - Use the client library
   - Implement payment flows
   - Handle receipts and verification

4. **Deploy to Production**
   - Set up proper infrastructure
   - Configure monitoring
   - Implement backup and recovery

## Resources

- [MPP-NEAR GitHub](https://github.com/Kampouse/mpp-near)
- [MPP-1.0 Specification](https://mpp.dev/spec)
- [NEAR Protocol](https://near.org/)
- [NEAR Intents](https://near.org/blog/introducing-near-intents/)
- [OutLayer API](https://outlayer.fastnear.com/)

## Support

If you encounter issues:

1. Check the [Troubleshooting](#troubleshooting) section
2. Review the [Examples](#detailed-examples)
3. Check the [GitHub Issues](https://github.com/Kampouse/mpp-near/issues)
4. Ask questions in the [Discord](https://discord.gg/near)

Happy testing! 🚀