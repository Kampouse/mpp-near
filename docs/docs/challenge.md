# Challenge

Payment requirements returned with HTTP 402.

## Overview

A Challenge represents payment requirements that the server returns with HTTP 402 (Payment Required) responses. It follows the MPP-1.0 specification.

## Creation

### Builder Pattern

```rust
use mpp_near::{Challenge, RequestData};

let request = RequestData::new("0.001", "wallet.near")
    .currency("USDC");

let challenge = Challenge::builder()
    .realm("api.example.com")
    .method("near-intents")
    .intent("charge")
    .request(request)
    .secret(b"hmac-secret")
    .ttl(300)
    .description("API access")
    .build()?;
```

## Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String | Auto | Unique identifier (HMAC or random) |
| `realm` | String | Yes | Protection space (e.g., "api.example.com") |
| `method` | String | Yes | Payment method (e.g., "near-intents") |
| `intent` | String | Yes | Payment intent (e.g., "charge", "session") |
| `request` | String | Yes | Base64url-encoded payment details |
| `expires` | String | Auto | RFC 3339 timestamp |
| `digest` | String | No | Request body digest (RFC 9530) |
| `description` | String | No | Human-readable description |

## Methods

### Serialization

```rust
// To WWW-Authenticate header
let www_auth = challenge.to_www_authenticate();

// To JSON
let json = serde_json::to_string(&challenge)?;
```

### Verification

```rust
// Verify HMAC binding
if challenge.verify_binding(b"hmac-secret") {
    // Challenge is authentic
}

// Check expiration
if challenge.is_expired() {
    // Challenge has expired
}
```

## See Also

- [Credential](./credential) - Payment proof
- [HMAC Binding](./hmac-binding) - Stateless verification
