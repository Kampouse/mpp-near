//! MPP Bridge - Cross-chain payment bridge for NEAR clients
//!
//! This module provides a bridge service that allows NEAR-based clients to pay
//! MPP servers on other chains (Ethereum, Solana, Bitcoin, etc.) using NEAR Intents.
//!
//! # How it works
//!
//! 1. Client receives 402 challenge from MPP server on Ethereum
//! 2. Client pays bridge via NEAR Intents (gasless)
//! 3. Bridge calls OutLayer cross-chain withdraw
//! 4. Bridge returns Ethereum tx hash
//! 5. Client submits standard MPP credential with Ethereum tx
//! 6. Server verifies normally (no NEAR knowledge needed)
//!
//! # Example
//!
//! ```rust,no_run
//! use mpp_near::bridge::{BridgeClient, BridgeRequest};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let bridge = BridgeClient::new("https://bridge.mpp.dev", "wk_your_key");
//!     
//!     let request = BridgeRequest {
//!         nonce: "abc123".into(),
//!         recipient: "0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb".into(),
//!         amount: 1.0,
//!         token: "usdc".into(),
//!         target_chain: "ethereum".into(),
//!     };
//!     
//!     let receipt = bridge.pay_and_bridge(request).await?;
//!     
//!     // Use receipt.tx_hash as MPP credential
//!     println!("Ethereum tx: {}", receipt.target_tx);
//!     
//!     Ok(())
//! }
//! ```

mod client;
mod server;
mod types;

pub use client::BridgeClient;
pub use server::{BridgeServer, BridgeConfig};
pub use types::{BridgeRequest, BridgeResponse, BridgeError, BridgeStatus};
