//! MPP-NEAR - Machine Payments Protocol with NEAR Intents
//!
//! This crate provides:
//! - **CLI**: Command-line tool for MPP payments
//! - **Client**: HTTP client for paying MPP endpoints
//! - **Primitives**: Spec-compliant MPP-1.0 types
//! - **Server**: Axum middleware (optional, feature: "server")
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use mpp_near::{Challenge, RequestData, Credential, Receipt};
//!
//! // Create a payment request
//! let request = RequestData::new("0.001", "wallet.near")
//!     .currency("USDC");
//!
//! // Create a challenge (server-side)
//! let challenge = Challenge::builder()
//!     .realm("api.example.com")
//!     .method("near-intents")
//!     .intent("charge")
//!     .request(request)
//!     .secret(b"hmac-secret")
//!     .build()?;
//!
//! // Client pays and creates credential
//! let credential = Credential::builder()
//!     .challenge(&challenge)
//!     .proof("intent_hash_from_payment")
//!     .build()?;
//!
//! // Server verifies and issues receipt
//! let receipt = Receipt::for_payment(&challenge.id, None, "0.001", "USDC");
//! # Ok::<(), mpp_near::Error>(())
//! ```

pub mod primitives;

#[cfg(feature = "server")]
pub mod middleware;

#[cfg(feature = "near-intents")]
pub mod near_intents;

// Re-export primitives for convenience
pub use primitives::{
    Challenge, ChallengeBuilder, RequestData,
    Credential, CredentialBuilder, ChallengeEcho,
    Receipt, ReceiptBuilder,
    Problem, ProblemType,
    Method, MethodRegistry, PaymentRequest, PaymentProof,
    BodyDigest,
    VerificationResult, Verifier,
    VERSION, DEFAULT_CHALLENGE_TTL,
    Error, Result,
};

// Re-export specific headers
pub use primitives::headers::{
    WWW_AUTHENTICATE, AUTHORIZATION, PAYMENT_RECEIPT,
    CONTENT_DIGEST, RETRY_AFTER, CACHE_CONTROL,
};
