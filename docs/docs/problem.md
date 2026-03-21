# Problem

RFC 9457 Problem Details for HTTP API errors.

## Overview

Problems provide detailed error information following the RFC 9457 standard. They are returned with HTTP 402 responses to explain why payment verification failed.

## Standard Problems

```rust
use mpp_near::{Problem, ProblemType};

// Payment required
let problem = Problem::payment_required();

// Insufficient payment
let problem = Problem::payment_insufficient("0.01", "0.001");

// Payment expired
let problem = Problem::payment_expired();

// Verification failed
let problem = Problem::verification_failed("Invalid signature");

// Unsupported method
let problem = Problem::method_unsupported("bitcoin");

// Malformed credential
let problem = Problem::malformed_credential("Missing proof");

// Invalid challenge
let problem = Problem::invalid_challenge("Challenge expired");
```

## Custom Problems

```rust
let problem = Problem::new(ProblemType::Custom("https://example.com/probs/rate-limit"))
    .with_detail("Rate limit exceeded")
    .with_status(429);
```

## Structure

```json
{
  "type": "https://paymentauth.org/problems/verification-failed",
  "title": "Verification Failed",
  "status": 402,
  "detail": "Invalid signature"
}
```

## Problem Types

| Type | Status | Description |
|------|--------|-------------|
| `payment-required` | 402 | Resource requires payment |
| `payment-insufficient` | 402 | Payment amount too low |
| `payment-expired` | 402 | Challenge or authorization expired |
| `verification-failed` | 402 | Payment proof invalid |
| `method-unsupported` | 402 | Payment method not accepted |
| `malformed-credential` | 402 | Invalid credential format |
| `invalid-challenge` | 402 | Challenge unknown, expired, or already used |

## Example

```rust
use mpp_near::{Credential, Problem};

match Credential::from_authorization(auth_header) {
    Ok(credential) => {
        // Verify payment
        if !verify_payment(&credential).await? {
            let problem = Problem::verification_failed("Invalid payment proof");
            return Ok(Response::json(&problem).with_status(402));
        }
        Ok(Response::json(&receipt))
    }
    Err(e) => {
        let problem = Problem::malformed_credential(&e.to_string());
        Ok(Response::json(&problem).with_status(402))
    }
}
```

## See Also

- [Challenge](./challenge) - Payment requirements
- [Credential](./credential) - Payment proof
