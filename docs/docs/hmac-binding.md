# HMAC Binding

Stateless challenge verification without storing challenges.

## Overview

HMAC binding allows servers to verify challenges without storing them in a database. The challenge ID is computed as an HMAC-SHA256 hash of all challenge parameters.

## How It Works

### 1. Create Challenge with Secret

```rust
use mpp_near::{Challenge, RequestData};

let secret = b"my-hmac-secret-key";

let challenge = Challenge::builder()
    .realm("api.example.com")
    .method("near-intents")
    .intent("charge")
    .request(RequestData::new("0.001", "wallet.near"))
    .secret(secret.to_vec())            // HMAC secret
    .build()?;

// Challenge ID is now: HMAC-SHA256(secret, binding_input)
```

### 2. Verify Without Storage

```rust
// Later, verify the challenge without storing it
if challenge.verify_binding(secret) {
    // Challenge is authentic - same secret was used
} else {
    // Challenge was not created with this secret
}
```

## Binding Input Format

The binding input is a pipe-delimited string of 7 positional slots:

```
realm|method|intent|request|expires|digest|opaque
```

### Example

```
api.example.com|near-intents|charge|eyJhbW91bnQiOiIwLjAwMSIsImN1cnJlbmN5IjoiVVNEQyIsInJlY2lwaWVudCI6IndhbGxldC5uZWFyIn0=|2025-01-15T12:05:00Z||
```

## Benefits

### Stateless Verification

```rust
// ✅ No database needed
// ✅ Scales horizontally
// ✅ No cleanup required

// Create challenge
let challenge = Challenge::builder()
    .realm("api.example.com")
    .method("near-intents")
    .intent("charge")
    .request(request)
    .secret(secret)
    .build()?;

// Verify later without storage
if challenge.verify_binding(secret) {
    // Authentic
}
```

### Tamper Detection

Any modification to challenge parameters invalidates the ID:

```rust
// Original challenge
let challenge = Challenge::builder()
    .realm("api.example.com")
    .method("near-intents")
    .intent("charge")
    .request(request)
    .secret(secret)
    .build()?;

// If attacker modifies amount
let modified = Challenge {
    amount: "1000.00".to_string(),  // Changed!
    ..challenge.clone()
};

// Verification fails
assert!(!modified.verify_binding(secret));
```

## Security

### Secret Management

```rust
// ✅ Good: Use environment variable
let secret = std::env::var("MPP_SECRET")
    .expect("MPP_SECRET must be set")
    .as_bytes();

// ✅ Good: Use strong secret
let secret = b"32-byte-random-secret-key-here-1234567890";

// ❌ Bad: Hardcoded weak secret
let secret = b"password123";
```

### Key Rotation

```rust
// Support multiple secrets for rotation
fn verify_challenge(challenge: &Challenge, secrets: &[&[u8]]) -> bool {
    secrets.iter().any(|secret| challenge.verify_binding(secret))
}

// Rotate keys
let old_secret = b"old-secret-key";
let new_secret = b"new-secret-key";

if verify_challenge(&challenge, &[new_secret, old_secret]) {
    // Accept during transition
}
```

## Example

```rust
use mpp_near::{Challenge, RequestData, Credential};

// Server creates challenge
let challenge = Challenge::builder()
    .realm("api.example.com")
    .method("near-intents")
    .intent("charge")
    .request(RequestData::new("0.001", "wallet.near"))
    .secret(b"server-secret")
    .build()?;

// Client pays and creates credential
let credential = Credential::builder()
    .challenge(&challenge)
    .proof("intent_hash_123")
    .build()?;

// Server verifies without storage
if !credential.verify_challenge_echo(&challenge) {
    return Err("Challenge mismatch");
}

if !challenge.verify_binding(b"server-secret") {
    return Err("Invalid challenge binding");
}

// Payment verified!
```

## See Also

- [Challenge](./challenge) - Payment requirements
- [Quick Start](./quick-start) - Get started
