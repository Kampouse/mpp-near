//! MPP-NEAR - Machine Payments Protocol with NEAR Intents
//!
//! This crate provides:
//! - **CLI**: Command-line tool for MPP payments
//! - **Client**: HTTP client for paying MPP endpoints
//! - **Agent Client**: Seamless auto-402 handling for autonomous agents
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
//!
//! ## Seamless Agent Client
//!
//! ```rust,no_run
//! use mpp_near::client::{AgentClient, BudgetConfig};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create client with budget limits
//!     let client = AgentClient::new("wk_your_api_key")
//!         .with_budget(BudgetConfig::new(0.10, 5.0)); // $0.10 per request, $5.00 per day
//!     
//!     // GET request - auto-handles 402 payment
//!     let data = client.get("https://paid-api.com/data").await?;
//!     
//!     // POST request - also auto-handles payment
//!     let result = client.post("https://paid-api.com/submit", &serde_json::json!({"key": "value"})).await?;
//!     
//!     Ok(())
//! }
//! ```

pub mod primitives;
pub mod types;

#[cfg(feature = "server")]
pub mod middleware;

#[cfg(feature = "server")]
pub mod server;

#[cfg(feature = "near-intents")]
pub mod near_intents;

// Client module (always available)
pub mod client;

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
