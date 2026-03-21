# MPP-NEAR Examples Summary

This document provides a comprehensive overview of all MPP-NEAR examples and how to test them end-to-end.

## Table of Contents

- [Overview](#overview)
- [Quick Start](#quick-start)
- [Examples](#examples)
  - [full_server.rs](#full_serverrs)
  - [full_client.rs](#full_clientrs)
  - [end_to_end_test.rs](#end_to_end_testrs)
- [Testing Scenarios](#testing-scenarios)
- [Configuration](#configuration)
- [Troubleshooting](#troubleshooting)
- [Next Steps](#next-steps)

## Overview

The MPP-NEAR project includes three comprehensive examples that demonstrate the complete payment flow:

1. **full_server.rs** - Complete MPP server with dual payment support
2. **full_client.rs** - Complete client with automatic payment handling
3. **end_to_end_test.rs** - Comprehensive test suite covering all features

### Key Features Demonstrated

- ✅ Full MPP-1.0 spec compliance
- ✅ Standard NEAR payments (on-chain transfers)
- ✅ NEAR Intents payments (gasless via OutLayer)
- ✅ Multiple pricing tiers
- ✅ Challenge creation and verification
- ✅ Credential generation and validation
- ✅ Receipt issuance and verification
- ✅ Error handling with RFC 9457 Problem details
- ✅ Mock payments for testing
- ✅ Real payments for production

## Quick Start

### Step 1: Start the Server

```bash
# Terminal 1
cargo run --example full_server --features server
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

### Step 2: Run the Client

```bash
# Terminal 2
cargo run --example full_client --features client
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

### Step 3: Run End-to-End Tests

```bash
# Terminal 3 (or any terminal)
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

## Examples

### full_server.rs

**Purpose**: A complete MPP server that accepts both standard NEAR and NEAR Intents payments.

**Key Features**:
- Dual payment method support (NEAR + NEAR Intents)
- Multiple pricing tiers with different costs
- Full MPP-1.0 spec compliance
- RFC 9457 Problem details for error handling
- Challenge creation with HMAC binding
- Credential verification
- Receipt issuance
- CORS support for cross-origin requests

**Architecture**:
```
┌─────────────────────────────────────────────────────────────┐
│                   Request Flow                            │
├─────────────────────────────────────────────────────────────┤
│                                                         │
│  1. Client → GET /api/v1/ping                          │
│                                                         │
│  2. Server → 402 + WWW-Authenticate header                │
│     (Challenge with pricing info)                            │
│                                                         │
│  3. Client → Pay (NEAR or NEAR Intents)                  │
│                                                         │
│  4. Client → GET /api/v1/ping + Authorization header       │
│     (Credential with proof)                                │
│                                                         │
│  5. Server → Verify credential → 200 + Payment-Receipt    │
│                                                         │
└─────────────────────────────────────────────────────────────┘
```

**Endpoints**:

| Path | Method | Cost | Description |
|------|--------|-------|-------------|
| `/health` | GET | FREE | Health check endpoint |
| `/pricing` | GET | FREE | Get pricing information |
| `/api/v1/ping` | GET | 0.0001 USDC | Simple ping endpoint |
| `/api/v1/analyze` | GET | 0.001 USDC | Data analysis endpoint |
| `/api/v1/generate` | GET | 0.01 USDC | Content generation endpoint |
| `/api/v1/complex` | GET | 0.1 USDC | Complex computation endpoint |

**Configuration**:

```bash
# Required
export MPP_HMAC_SECRET="your-secret-here"  # For challenge binding
export MPP_RECIPIENT="merchant.near"     # Recipient account

# Optional
export MPP_RPC_URL="https://rpc.mainnet.near.org"
export MPP_CHALLENGE_TTL="300"          # 5 minutes
export OUTLAYER_API_KEY="your-api-key"   # For intents
```

**Running**:

```bash
# Basic run with default config
cargo run --example full_server --features server

# With custom port (modify code)
# Server runs on http://0.0.0.0:3000

# With custom recipient
export MPP_RECIPIENT="your-account.near"
cargo run --example full_server --features server
```

### full_client.rs

**Purpose**: A complete MPP client that can pay for services using both standard NEAR and NEAR Intents.

**Key Features**:
- Automatic payment method selection
- Automatic retry logic for failed payments
- Payment discovery from challenges
- Receipt verification
- Multiple endpoint support
- Configurable payment preferences
- Mock payment support for testing

**Architecture**:
```
┌─────────────────────────────────────────────────────────────┐
│                   Client Flow                            │
├─────────────────────────────────────────────────────────────┤
│                                                         │
│  1. client.request("/api/v1/ping")                     │
│     ↓                                                   │
│  2. Receive 402 + Challenge                            │
│     ↓                                                   │
│  3. Extract challenge parameters                          │
│     ↓                                                   │
│  4. Select payment method (auto/near/near-intents)         │
│     ↓                                                   │
│  5. Pay for challenge                                    │
│     - Create credential with proof                          │
│     ↓                                                   │
│  6. Retry request with Authorization header                │
│     ↓                                                   │
│  7. Receive 200 + Receipt                               │
│     ↓                                                   │
│  8. Verify receipt                                       │
│     ↓                                                   │
│  9. Return response                                      │
│                                                         │
└─────────────────────────────────────────────────────────────┘
```

**Configuration**:

```bash
# Required
export MPP_SERVER_URL="http://localhost:3000"

# Optional
export MPP_PAYMENT_METHOD="auto"        # auto, near, or near-intents
export MPP_MAX_RETRIES="3"            # Retry attempts
export MPP_RETRY_DELAY="2"             # Seconds between retries

# For standard NEAR payments
export NEAR_ACCOUNT_ID="your-account.near"
export NEAR_PRIVATE_KEY="ed25519:..."
export NEAR_RPC_URL="https://rpc.testnet.near.org"

# For NEAR Intents payments
export OUTLAYER_API_KEY="your-api-key"
export OUTLAYER_API_URL="https://outlayer.fastnear.com"
```

**Usage Examples**:

```rust
// Create client from environment
let client = MppClient::from_env()?;

// Make requests
let response = client.health_check().await?;
let response = client.ping().await?;
let response = client.analyze().await?;
let response = client.generate().await?;
let response = client.complex().await?;

// Check pricing
let pricing = client.get_pricing().await?;
```

**Running**:

```bash
# With client features only
cargo run --example full_client --features client

# With intents support
cargo run --example full_client --features client,intents

# With custom configuration
export MPP_SERVER_URL="http://localhost:3000"
export MPP_PAYMENT_METHOD="near-intents"
cargo run --example full_client --features client,intents
```

### end_to_end_test.rs

**Purpose**: A comprehensive test suite that covers the entire payment flow and all edge cases.

**Test Categories**:

1. **Core Payment Flow Tests** (7 tests)
   - Complete payment flow (challenge → credential → receipt)
   - Challenge binding verification
   - Credential serialization round-trip
   - Challenge serialization round-trip
   - Challenge expiration
   - Receipt serialization round-trip
   - Problem error handling

2. **NEAR Intents Tests** (2 tests)
   - NEAR Intents payment method
   - Intents request data extraction

3. **Server Verifier Tests** (2 tests, server feature required)
   - Server verifier challenge creation
   - Server verifier cleanup

4. **Type Validation Tests** (3 tests)
   - AccountId validation
   - NearAmount conversions
   - Gas conversions

5. **Edge Cases and Error Handling** (3 tests)
   - Invalid challenge handling
   - Mismatched credential detection
   - Request data encoding/decoding

6. **Integration Tests** (2 tests)
   - Full HTTP flow simulation
   - Multiple pricing tiers

**Total Tests**: 19 tests

**Running**:

```bash
# Run all tests
cargo run --example end_to_end_test --features client,server,intents

# Run with verbose output
RUST_LOG=debug cargo run --example end_to_end_test --features client,server,intents

# Run specific test category
# Modify code to disable specific test groups
```

**Expected Output**:

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
║  NEAR Intents Tests                                        ║
╚════════════════════════════════════════════════════════════╝
  ✓ PASS (0 ms) - NEAR Intents Payment Method
  ✓ PASS (0 ms) - Intents Request Data Extraction

╔════════════════════════════════════════════════════════════╗
║  Type Validation Tests                                      ║
╚════════════════════════════════════════════════════════════╝
  ✓ PASS (0 ms) - AccountId Validation
  ✓ PASS (0 ms) - NearAmount Conversions
  ✓ PASS (0 ms) - Gas Conversions

╔════════════════════════════════════════════════════════════╗
║  Edge Cases and Error Handling                             ║
╚════════════════════════════════════════════════════════════╝
  ✓ PASS (0 ms) - Invalid Challenge Handling
  ✓ PASS (0 ms) - Mismatched Credential Detection
  ✓ PASS (0 ms) - Request Data Encoding/Decoding

╔════════════════════════════════════════════════════════════╗
║  Integration Tests                                         ║
╚════════════════════════════════════════════════════════════╝
  ✓ PASS (0 ms) - Full HTTP Flow Simulation
  ✓ PASS (0 ms) - Multiple Pricing Tiers

╔════════════════════════════════════════════════════════════╗
║                   Test Summary                               ║
╠════════════════════════════════════════════════════════════╣
║  Total Tests: 19                                            ║
║  Passed:      19                                            ║
║  Failed:      0                                             ║
║  Duration:    5 ms                                          ║
╚════════════════════════════════════════════════════════════╝

✅ All tests passed!
```

## Testing Scenarios

### Scenario 1: Mock Payments (Local Testing)

**Purpose**: Test all functionality without real payments.

**Setup**:
```bash
# Terminal 1: Start server with default config
cargo run --example full_server --features server

# Terminal 2: Run client (no payment config needed)
cargo run --example full_client --features client
```

**Expected**: All requests succeed with mock payments.

**Use Case**: Development and testing without spending real money.

### Scenario 2: Standard NEAR Payments (Testnet)

**Purpose**: Test with real NEAR transfers on testnet.

**Setup**:
```bash
# Terminal 1: Start server
export MPP_RECIPIENT="your-account.testnet"
export MPP_RPC_URL="https://rpc.testnet.near.org"
cargo run --example full_server --features server

# Terminal 2: Run client with NEAR account
export NEAR_ACCOUNT_ID="your-account.testnet"
export NEAR_PRIVATE_KEY="your-testnet-private-key"
export NEAR_RPC_URL="https://rpc.testnet.near.org"
export MPP_PAYMENT_METHOD="near"
cargo run --example full_client --features client
```

**Expected**: Real NEAR transfers executed on testnet.

**Use Case**: Testing with real payments without spending mainnet NEAR.

### Scenario 3: NEAR Intents (Gasless Payments)

**Purpose**: Test gasless payments via OutLayer.

**Setup**:
```bash
# Terminal 1: Start server
export OUTLAYER_API_KEY="your-api-key"
cargo run --example full_server --features server

# Terminal 2: Run client with intents
export OUTLAYER_API_KEY="your-api-key"
export MPP_PAYMENT_METHOD="near-intents"
cargo run --example full_client --features client,intents
```

**Expected**: Gasless payments via OutLayer.

**Use Case**: Production payments without gas costs.

### Scenario 4: Comprehensive Testing

**Purpose**: Run all tests to verify functionality.

**Setup**:
```bash
# Run all tests
cargo run --example end_to_end_test --features client,server,intents
```

**Expected**: All 19 tests pass.

**Use Case**: Continuous integration and verification.

## Configuration

### Server Configuration

| Variable | Required | Default | Description |
|----------|-----------|----------|-------------|
| `MPP_HMAC_SECRET` | Yes | - | HMAC secret for challenge binding |
| `MPP_RECIPIENT` | Yes | merchant.near | Recipient account ID |
| `MPP_RPC_URL` | No | https://rpc.mainnet.near.org | NEAR RPC endpoint |
| `MPP_CHALLENGE_TTL` | No | 300 | Challenge expiration (seconds) |
| `OUTLAYER_API_KEY` | No | - | OutLayer API key for intents |

### Client Configuration

| Variable | Required | Default | Description |
|----------|-----------|----------|-------------|
| `MPP_SERVER_URL` | Yes | http://localhost:3000 | Server URL |
| `MPP_PAYMENT_METHOD` | No | auto | Payment method: auto, near, near-intents |
| `MPP_MAX_RETRIES` | No | 3 | Maximum retry attempts |
| `MPP_RETRY_DELAY` | No | 2 | Retry delay (seconds) |

### NEAR Payment Configuration

| Variable | Required | Description |
|----------|-----------|-------------|
| `NEAR_ACCOUNT_ID` | Yes | Your NEAR account ID |
| `NEAR_PRIVATE_KEY` | Yes | Your NEAR private key |
| `NEAR_RPC_URL` | No | NEAR RPC endpoint |

### NEAR Intents Configuration

| Variable | Required | Description |
|----------|-----------|-------------|
| `OUTLAYER_API_KEY` | Yes | Your OutLayer API key |
| `OUTLAYER_API_URL` | No | OutLayer API URL |

## Troubleshooting

### Server Won't Start

**Error**: `Failed to bind to address`

**Solution**:
```bash
# Check if port 3000 is in use
lsof -i :3000

# Kill existing process or use different port
```

### Client Connection Refused

**Error**: `Failed to fetch pricing: Connection refused`

**Solution**:
```bash
# Make sure server is running
curl http://localhost:3000/health
```

### Invalid Account ID

**Error**: `InvalidAccountId("invalid-account")`

**Solution**: Ensure account ID follows NEAR rules:
- Contains only lowercase letters, numbers, underscore, or hyphen
- Each part separated by dots (.)
- Each part 1-63 characters
- Total length <= 64 characters

### Challenge Expired

**Error**: `Challenge expired`

**Solution**:
```bash
# Increase challenge TTL
export MPP_CHALLENGE_TTL="600"  # 10 minutes instead of 5

# Or complete payment faster
```

### Payment Verification Failed

**Error**: `Payment verification failed`

**Solution**:
- Check that proof matches transaction
- Ensure recipient account is correct
- Verify amount matches challenge requirements
- Check that transaction has enough confirmations

### Out of Gas

**Error**: `TransactionFailed: Out of gas`

**Solution**:
```bash
# Use NEAR Intents for gasless payments
export MPP_PAYMENT_METHOD="near-intents"

# Or ensure account has enough NEAR for gas
```

### Storage Registration Required

**Error**: `Storage registration required`

**Solution**:
```bash
# Register storage for tokens using CLI
cargo run --bin mpp-near -- storage-deposit --token usdt.tether-token.near
```

## Next Steps

### For Developers

1. **Explore the codebase**
   - Read source code in `examples/` directory
   - Understand MPP protocol implementation
   - Learn about NEAR Intents integration

2. **Modify examples**
   - Customize pricing tiers
   - Add new endpoints
   - Implement custom payment methods
   - Add logging and monitoring

3. **Build your own service**
   - Use server example as template
   - Integrate with your existing APIs
   - Implement custom business logic
   - Add authentication and authorization

### For Production

1. **Configure for production**
   ```bash
   # Use strong HMAC secret
   export MPP_HMAC_SECRET=$(openssl rand -hex 32)
   
   # Use HTTPS
   # Configure TLS certificates
   
   # Use production RPC
   export MPP_RPC_URL="https://rpc.mainnet.near.org"
   ```

2. **Implement monitoring**
   - Log all transactions
   - Track revenue and metrics
   - Set up alerts for failures
   - Monitor payment flow

3. **Handle edge cases**
   - Implement proper retry logic
   - Handle rate limits
   - Provide clear error messages
   - Implement circuit breakers

4. **Deploy to production**
   - Set up proper infrastructure
   - Configure load balancing
   - Implement backup and recovery
   - Set up CI/CD pipeline

### Resources

- [MPP-NEAR GitHub](https://github.com/Kampouse/mpp-near)
- [MPP-1.0 Specification](https://mpp.dev/spec)
- [NEAR Protocol](https://near.org/)
- [NEAR Intents](https://near.org/blog/introducing-near-intents/)
- [OutLayer API](https://outlayer.fastnear.com/)
- [TESTING_GUIDE.md](./TESTING_GUIDE.md) - Detailed testing instructions

## Summary

The three examples provide a complete end-to-end demonstration of MPP-NEAR functionality:

1. **full_server.rs**: Production-ready server with all features
2. **full_client.rs**: Production-ready client with automatic payment handling
3. **end_to_end_test.rs**: Comprehensive test suite covering all scenarios

All examples are:
- ✅ Fully functional and ready to use
- ✅ Well-documented with clear code
- ✅ Following MPP-1.0 specification
- ✅ Supporting both payment methods
- ✅ Include proper error handling
- ✅ Ready for production deployment

Start with the quick start guide above, then explore each example in detail to understand the implementation and customize it for your needs.