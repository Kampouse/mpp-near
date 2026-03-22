# MPP-NEAR Examples - Fixed to Work with Real Payments

## What Was Fixed

Your examples were using **mock/demo payments** instead of real blockchain payments. Here's what changed:

### 1. Server (`full_server.rs`)
- **Before**: `.with_mocks()` - accepted fake payments
- **After**: Requires real payment verification via OutLayer API or NEAR blockchain

### 2. Client (`full_client.rs`)
- **Before**: Used `mock_tx_` proofs (fake)
- **After**: Implements real payment logic:
  - `create_near_intent()` - Calls OutLayer API to create real intents
  - `submit_near_transaction()` - Submits real NEAR transactions

### 3. Test Client (`test_payment_client.rs`)
- **Before**: Mock transaction hash
- **After**: Real OutLayer API call to create intents

## How to Use

### Option 1: NEAR Intents (Recommended - Gasless)

1. Get an OutLayer API key from https://outlayer.fastnear.com
2. Add to your `.env` file:
   ```bash
   OUTLAYER_API_KEY=wk_your_api_key_here
   ```
3. Run the examples:
   ```bash
   # Terminal 1: Start server
   cargo run --example full_server --features server

   # Terminal 2: Run client
   cargo run --example full_client --features client,intents
   ```

### Option 2: Standard NEAR Payments

1. Have a NEAR account with NEAR tokens
2. Add to your `.env` file:
   ```bash
   NEAR_ACCOUNT_ID=your-account.near
   NEAR_PRIVATE_KEY=ed25519:your_private_key_here
   NEAR_RPC_URL=https://rpc.testnet.near.org  # or mainnet
   ```
3. Run the examples:
   ```bash
   cargo run --example full_client --features client
   ```

## Quick Test

Run the test script to verify everything works:

```bash
./test_examples.sh
```

## What Makes This Work

### Real Payment Flow

**Server** creates challenge → **Client** pays via OutLayer API → **Server** verifies payment

1. Server creates a payment challenge with amount and recipient
2. Client extracts challenge from WWW-Authenticate header
3. Client calls OutLayer API with payment details
4. OutLayer returns an intent hash (real transaction proof)
5. Client creates credential with the intent hash
6. Server verifies the intent hash with OutLayer API
7. Server serves the paid content with receipt

### No More Mocks

The old code used:
- `mock_tx_` - obviously fake
- `with_mocks()` - server accepted any fake payment
- Commented out real payment logic

The new code uses:
- Real OutLayer API calls
- Real NEAR blockchain transactions
- Actual payment verification
- Working implementation (not comments!)

## Environment Variables Required

For NEAR Intents (recommended):
```bash
OUTLAYER_API_KEY=wk_your_key
```

For Standard NEAR:
```bash
NEAR_ACCOUNT_ID=your-account.near
NEAR_PRIVATE_KEY=ed25519:your_key
NEAR_RPC_URL=https://rpc.mainnet.near.org
```

Server config:
```bash
MPP_RECIPIENT=merchant.near
MPP_HMAC_SECRET=your-secret-here
```

## Troubleshooting

### "OUTLAYER_API_KEY not set"
You need to get an API key from OutLayer to use NEAR Intents (gasless payments).

### "Payment verification failed"
Make sure:
- Your OutLayer API key is valid
- The intent hash is real (from OutLayer API)
- The recipient account exists

### "Failed to bind to address"
Port 3000 is already in use. Kill the existing process or change the port.

## Files Changed

- `examples/full_server.rs` - Removed `.with_mocks()`, fixed verification logic
- `examples/full_client.rs` - Implemented real payment functions
- `examples/test_payment_client.rs` - Real OutLayer API calls
- `test_examples.sh` - New test script to verify everything works
