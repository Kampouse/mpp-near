//! MPP Primitives - Spec-compliant types
//!
//! This module contains the core MPP-1.0 spec types:
//! - Challenge: Payment requirements
//! - Credential: Payment proof
//! - Receipt: Payment confirmation
//! - Problem: RFC 9457 errors
//! - Method: Payment method trait
//! - BodyDigest: RFC 9530 digest
//! - Verifier: Verification trait

pub mod challenge;
pub mod credential;
pub mod receipt;
pub mod problem;
pub mod method;
pub mod digest;
pub mod verify;
pub mod headers;

// Re-export core types
pub use challenge::{Challenge, ChallengeBuilder, RequestData};
pub use credential::{Credential, CredentialBuilder, ChallengeEcho};
pub use receipt::{Receipt, ReceiptBuilder};
pub use problem::{Problem, ProblemType};
pub use method::{Method, MethodRegistry, PaymentRequest, PaymentProof};
pub use digest::BodyDigest;
pub use verify::{VerificationResult, Verifier};
pub use headers::*;

/// MPP protocol version
pub const VERSION: &str = "MPP/1.0";

/// Default challenge TTL in seconds
pub const DEFAULT_CHALLENGE_TTL: i64 = 300; // 5 minutes

/// Error type for MPP operations
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Invalid challenge format
    #[error("Invalid challenge: {0}")]
    InvalidChallenge(String),
    
    /// Invalid credential format
    #[error("Invalid credential: {0}")]
    InvalidCredential(String),
    
    /// Challenge expired
    #[error("Challenge expired")]
    ChallengeExpired,
    
    /// Challenge not found
    #[error("Challenge not found")]
    ChallengeNotFound,
    
    /// Payment verification failed
    #[error("Payment verification failed: {0}")]
    VerificationFailed(String),
    
    /// Unsupported payment method
    #[error("Unsupported payment method: {0}")]
    UnsupportedMethod(String),
    
    /// Invalid amount
    #[error("Invalid amount: {0}")]
    InvalidAmount(String),
    
    /// HTTP error
    #[error("HTTP error: {0}")]
    Http(#[from] http::Error),
    
    /// JSON error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    /// Base64 decode error
    #[error("Base64 decode error: {0}")]
    Base64(#[from] base64::DecodeError),
    
    /// Other error
    #[error("{0}")]
    Other(String),
}

/// Result type for MPP operations
pub type Result<T> = std::result::Result<T, Error>;
