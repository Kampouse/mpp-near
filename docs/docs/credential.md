# Credential

Payment proof submitted by the client.

## Overview

A Credential proves that payment has been made. It contains the challenge being responded to, a payment proof (method-specific), and optional additional fields.

## Creation

### Builder Pattern

```rust
use mpp_near::Credential;

let credential = Credential::builder()
    .challenge(&challenge)              // From original challenge
    .proof("intent_hash_123")           // Payment proof/tx hash
    .source("did:near:user.near")       // Optional: Payer identifier
    .build()?;
```

## Structure

The credential is base64url-encoded JSON following MPP-1.0 spec:

```json
{
  "challenge": {
    "id": "abc123",
    "realm": "api.example.com",
    "method": "near-intents",
    "intent": "charge",
    "request": "eyJhbW91bnQiOi..."
  },
  "source": "did:near:user.near",
  "payload": {
    "proof": "intent_hash_123"
  }
}
```

## Fields

### Challenge Echo

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String | Yes | Challenge identifier |
| `realm` | String | Yes | Protection space |
| `method` | String | Yes | Payment method |
| `intent` | String | Yes | Payment intent |
| `request` | String | Yes | Base64url-encoded request |
| `description` | String | No | Description from challenge |
| `opaque` | String | No | Server correlation data |
| `digest` | String | No | Body digest from challenge |
| `expires` | String | No | Expiration timestamp |

### Credential Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `challenge` | Object | Yes | Echoed challenge parameters |
| `source` | String | No | Payer identifier (DID format recommended) |
| `payload` | Object | Yes | Payment proof |

## Methods

### Serialization

```rust
// To Authorization header
let auth = credential.to_authorization();
// Output: Payment eyJjaGFsbGVuZ2UiOnsiaWQiOi...

// Parse from Authorization header
let credential = Credential::from_authorization(&auth)?;
```

### Parsing

```rust
// From HTTP headers
let credential = Credential::from_headers(&headers)?;

// From Authorization header string
let credential = Credential::from_authorization(auth_header)?;
```

### Verification

```rust
// Verify challenge echo matches original
if credential.verify_challenge_echo(&challenge) {
    // Challenge parameters match
} else {
    // Mismatch - possible replay attack
}
```

## Challenge Echo

The credential must echo back the challenge parameters to prevent replay attacks:

```rust
// All required fields must match
assert_eq!(credential.challenge.id, challenge.id);
assert_eq!(credential.challenge.realm, challenge.realm);
assert_eq!(credential.challenge.method, challenge.method);
assert_eq!(credential.challenge.intent, challenge.intent);
assert_eq!(credential.challenge.request, challenge.request);

// Optional fields must match if present in challenge
if let Some(ref desc) = challenge.description {
    assert_eq!(credential.challenge.description, Some(desc.clone()));
}
```

## Payment Proof

The `payload.proof` field contains method-specific payment proof:

### NEAR Intents

```rust
// Intent hash from NEAR Intents payment
let credential = Credential::builder()
    .challenge(&challenge)
    .proof("4dRBrPj8ouGe7sfR794rvHwqbBCSnPAbGqULprXyc9eA")
    .source("did:near:user.near")
    .build()?;
```

### Test Payments

```rust
// Test payments (for development)
let credential = Credential::builder()
    .challenge(&challenge)
    .proof("test_payment_123")  // or "mock_*" or "fake_*"
    .build()?;

if credential.is_mock() {
    // Accept in test mode
}
```

## Example

```rust
use mpp_near::{Challenge, Credential, RequestData};

// Server creates challenge
let challenge = Challenge::builder()
    .realm("api.example.com")
    .method("near-intents")
    .intent("charge")
    .request(RequestData::new("0.001", "wallet.near"))
    .secret(b"secret")
    .build()?;

// Client pays via NEAR Intents
let intent_hash = pay_via_near_intents(&challenge)?;

// Client creates credential
let credential = Credential::builder()
    .challenge(&challenge)
    .proof(intent_hash)
    .source("did:near:user.near")
    .build()?;

// Client retries with credential
let response = client
    .get("/api/resource")
    .header("Authorization", credential.to_authorization())
    .send()?;

// Server verifies
if !credential.verify_challenge_echo(&challenge) {
    return Err("Challenge mismatch");
}
```

## Security

### Challenge Echo Verification

Always verify the challenge echo to prevent replay attacks:

```rust
// ✅ Good: Verify echo
if !credential.verify_challenge_echo(&original_challenge) {
    return Err("Invalid credential");
}

// ❌ Bad: Skip verification
// Vulnerable to replay attacks
```

### Mock Payment Detection

```rust
// Check if test payment
if credential.is_mock() {
    if !config.accept_mocks {
        return Err("Test payments not accepted");
    }
}
```

## See Also

- [Challenge](./challenge) - Payment requirements
- [Receipt](./receipt) - Payment confirmation
- [Payment Methods](./payment-methods) - Available methods
