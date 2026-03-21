# Receipt

Payment confirmation returned by the server.

## Overview

A Receipt confirms that payment was successfully verified. It includes details about the payment for the client's records.

## Creation

### Simple Creation

```rust
use mpp_near::Receipt;

let receipt = Receipt::for_payment(
    &challenge.id,                      // Challenge ID
    Some("user.near"),                  // Account (optional)
    "0.001",                            // Amount
    "USDC"                              // Token
);
```

### Builder Pattern

```rust
let receipt = Receipt::builder()
    .challenge_id(&challenge.id)
    .account("user.near")
    .amount("0.001")
    .token("USDC")
    .status("confirmed")
    .proof("tx_hash_123")               // Optional: Transaction hash
    .build()?;
```

## Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String | Auto | Unique receipt ID |
| `challenge_id` | String | Yes | Original challenge ID |
| `account` | String | No | Payer account |
| `amount` | String | Yes | Amount paid |
| `token` | String | Yes | Token paid |
| `status` | String | Auto | Payment status (default: "confirmed") |
| `timestamp` | i64 | Auto | Unix timestamp |
| `proof` | String | No | Transaction hash |

## Methods

### Serialization

```rust
// To Payment-Receipt header
let header = receipt.to_header();
// Output: Payment id="xyz", challengeId="abc", amount="0.001", ...

// Parse from header
let receipt = Receipt::from_header(&header)?;
```

### JSON

```rust
// Serialize to JSON
let json = serde_json::to_string(&receipt)?;

// Deserialize from JSON
let receipt: Receipt = serde_json::from_str(&json)?;
```

## Example

```rust
use mpp_near::{Challenge, Credential, Receipt, RequestData};

// Verify payment
let credential = Credential::from_authorization(auth_header)?;

if credential.verify_challenge_echo(&challenge) 
    && challenge.verify_binding(b"secret") {
    
    // Issue receipt
    let receipt = Receipt::builder()
        .challenge_id(&challenge.id)
        .account(credential.source.as_deref())
        .amount("0.001")
        .token("USDC")
        .proof(&credential.proof)
        .build()?;
    
    // Return 200 with receipt
    Response::builder()
        .status(200)
        .header("Payment-Receipt", receipt.to_header())
        .json(&receipt)
        .build()
} else {
    // Return 402
    Response::builder()
        .status(402)
        .json(&Problem::verification_failed("Invalid payment"))
        .build()
}
```

## Receipt Structure

```json
{
  "id": "b1d3c7900530c539035bce3eab9f4c02",
  "challengeId": "abc123",
  "account": "user.near",
  "amount": "0.001",
  "token": "USDC",
  "status": "confirmed",
  "timestamp": 1704067200,
  "proof": "intent_hash_123"
}
```

## See Also

- [Challenge](./challenge) - Payment requirements
- [Credential](./credential) - Payment proof
- [Problem](./problem) - Error handling
