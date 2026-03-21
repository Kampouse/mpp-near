# Quick Start

Get started with MPP-NEAR in 5 minutes.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
mpp-near = { git = "https://github.com/Kampouse/mpp-near" }
```

## Basic Usage

### 1. Create Challenge

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
    .build()?;

// Return 402 with WWW-Authenticate
let www_auth = challenge.to_www_authenticate();
```

### 2. Verify Payment

```rust
use mpp_near::Credential;

let credential = Credential::from_authorization(auth_header)?;

if credential.verify_challenge_echo(&challenge)
    && challenge.verify_binding(b"hmac-secret") {
    // Payment valid
}
```

### 3. Issue Receipt

```rust
use mpp_near::Receipt;

let receipt = Receipt::for_payment(
    &challenge.id,
    None,
    "0.001",
    "USDC"
);
```

## Next Steps

- [Challenge](./challenge) - Payment requirements
- [Credential](./credential) - Payment proof
- [Receipt](./receipt) - Payment confirmation
- [Examples](./examples) - Complete examples
