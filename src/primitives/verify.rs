//! MPP Verifier - Payment verification trait and result types

use crate::{Challenge, Credential, Method, Error, Result};

/// Result of payment verification
#[derive(Debug, Clone, PartialEq)]
pub enum VerificationResult {
    /// Payment is valid
    Valid {
        /// Amount that was paid
        amount: String,
        /// Token that was paid
        token: String,
        /// Account that paid (optional)
        account: Option<String>,
        /// Transaction/proof hash (optional)
        proof: Option<String>,
    },
    /// Payment is invalid
    Invalid {
        /// Reason for failure
        reason: String,
    },
    /// Challenge expired
    Expired,
    /// Challenge not found
    NotFound,
    /// Credential malformed
    MalformedCredential {
        /// Reason
        reason: String,
    },
}

impl VerificationResult {
    /// Create a valid result
    pub fn valid(amount: impl Into<String>, token: impl Into<String>) -> Self {
        Self::Valid {
            amount: amount.into(),
            token: token.into(),
            account: None,
            proof: None,
        }
    }
    
    /// Create a valid result with account
    pub fn valid_with_account(
        amount: impl Into<String>,
        token: impl Into<String>,
        account: impl Into<String>,
    ) -> Self {
        Self::Valid {
            amount: amount.into(),
            token: token.into(),
            account: Some(account.into()),
            proof: None,
        }
    }
    
    /// Create an invalid result
    pub fn invalid(reason: impl Into<String>) -> Self {
        Self::Invalid {
            reason: reason.into(),
        }
    }
    
    /// Check if the result is valid
    pub fn is_valid(&self) -> bool {
        matches!(self, Self::Valid { .. })
    }
    
    /// Get the amount if valid
    pub fn amount(&self) -> Option<&str> {
        match self {
            Self::Valid { amount, .. } => Some(amount),
            _ => None,
        }
    }
    
    /// Get the error reason if invalid
    pub fn error_reason(&self) -> Option<&str> {
        match self {
            Self::Invalid { reason } => Some(reason),
            Self::MalformedCredential { reason } => Some(reason),
            _ => None,
        }
    }
}

/// Verifier trait for payment verification
///
/// Implement this trait to provide custom verification logic.
/// The default implementation uses a Method trait object.
#[async_trait::async_trait]
pub trait Verifier: Send + Sync {
    /// Verify a credential against a challenge
    async fn verify(
        &self,
        challenge: &Challenge,
        credential: &Credential,
    ) -> Result<VerificationResult>;
}

/// Default verifier using a Method
pub struct MethodVerifier<M: Method> {
    method: M,
}

impl<M: Method> MethodVerifier<M> {
    /// Create a new verifier with a method
    pub fn new(method: M) -> Self {
        Self { method }
    }
}

#[async_trait::async_trait]
impl<M: Method + Send + Sync> Verifier for MethodVerifier<M> {
    async fn verify(
        &self,
        challenge: &Challenge,
        credential: &Credential,
    ) -> Result<VerificationResult> {
        // Check expiration
        if challenge.is_expired() {
            return Ok(VerificationResult::Expired);
        }
        
        // Verify challenge echo
        if !credential.verify_challenge_echo(challenge) {
            return Ok(VerificationResult::MalformedCredential {
                reason: "Challenge echo mismatch".into(),
            });
        }
        
        // Verify via method
        let valid = self.method.verify_credential(challenge, credential).await?;
        
        if valid {
            // Extract request data to get amount/token
            let request_data = crate::RequestData::decode(&challenge.request)?;
            Ok(VerificationResult::valid(
                request_data.amount,
                request_data.currency.unwrap_or_else(|| "UNKNOWN".to_string()),
            ))
        } else {
            Ok(VerificationResult::invalid("Payment verification failed"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RequestData, ChallengeBuilder, CredentialBuilder};
    use async_trait::async_trait;
    
    struct TestVerifier;
    
    #[async_trait]
    impl Verifier for TestVerifier {
        async fn verify(
            &self,
            challenge: &Challenge,
            _credential: &Credential,
        ) -> Result<VerificationResult> {
            let request_data = RequestData::decode(&challenge.request)?;
            Ok(VerificationResult::valid(
                request_data.amount,
                request_data.currency.unwrap_or("TEST".to_string()),
            ))
        }
    }
    
    #[tokio::test]
    async fn test_verifier() {
        let verifier = TestVerifier;
        let request = RequestData::new("1.0", "wallet.near")
            .currency("USDC");
        
        let challenge = Challenge::builder()
            .realm("test")
            .method("test")
            .intent("charge")
            .request(request)
            .secret(b"test-secret".to_vec())
            .build()
            .unwrap();
        
        let credential = Credential::builder()
            .challenge(&challenge)
            .proof("test_proof")
            .build()
            .unwrap();
        
        let result = verifier.verify(&challenge, &credential).await.unwrap();
        assert!(result.is_valid());
        assert_eq!(result.amount(), Some("1.0"));
    }
}
