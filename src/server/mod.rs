//! Server-side NEAR payment verification

mod verifier;

pub use verifier::{NearVerifier, VerifierConfig};

/// Payment credential for extraction
#[derive(Debug, Clone)]
pub struct NearPayment {
    pub credential: crate::types::NearCredential,
}

impl NearPayment {
    /// Get payer account ID
    pub fn payer(&self) -> &str {
        self.credential.payer.as_str()
    }
    
    /// Get payment amount
    pub fn amount(&self) -> String {
        self.credential.amount.to_string()
    }
    
    /// Get transaction hash
    pub fn tx_hash(&self) -> &str {
        self.credential.tx_hash.as_str()
    }
}
