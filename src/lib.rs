//! NEAR payment provider for Machine Payments Protocol (MPP)
//!
//! This crate extends mpp-rs with NEAR blockchain support, enabling:
//! - HTTP 402 payments with NEAR tokens
//! - NEP-141 token payments (USDC, etc.)
//! - Gasless payments via NEAR Intents
//!
//! # Example
//!
//! ```rust,no_run
//! use mpp_near::client::{NearProvider, NearConfig};
//! use reqwest_middleware::ClientBuilder;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let provider = NearProvider::new(
//!         "kampouse.near".parse()?,
//!         "ed25519:...".parse()?,
//!         "https://rpc.mainnet.near.org",
//!     )?;
//!     
//!     let client = ClientBuilder::new(reqwest::Client::new())
//!         .with(mpp_near::client::PaymentMiddleware::new(provider))
//!         .build();
//!     
//!     // Automatically handles 402 responses with NEAR payment
//!     let resp = client.get("https://api.example.com/paid").send().await?;
//!     Ok(())
//! }
//! ```

pub mod types;
pub mod client;
pub mod server;

pub use types::{AccountId, Gas, NearAmount, TransactionHash};
pub use client::{NearProvider, NearConfig, PaymentMiddleware};
pub use server::{NearVerifier, NearPayment};

/// Errors for NEAR MPP operations
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid account ID: {0}")]
    InvalidAccountId(String),
    
    #[error("Invalid signature: {0}")]
    InvalidSignature(String),
    
    #[error("RPC error: {0}")]
    RpcError(String),
    
    #[error("Transaction failed: {0}")]
    TransactionFailed(String),
    
    #[error("Insufficient balance: required {required}, available {available}")]
    InsufficientBalance { required: String, available: String },
    
    #[error("Payment verification failed: {0}")]
    VerificationFailed(String),
    
    #[error("Invalid challenge: {0}")]
    InvalidChallenge(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
