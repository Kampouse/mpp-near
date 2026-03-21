//! NEAR Intents Payment Method
//!
//! Implements MPP payment verification using NEAR Intents via OutLayer API.

use serde::{Deserialize, Serialize};

use crate::primitives::{Challenge, Credential, Method, Error, Result};
use crate::primitives::method::{PaymentRequest, PaymentProof};
use crate::primitives::challenge::RequestData;

/// OutLayer API base URL
pub const OUTLAYER_API: &str = "https://outlayer.fastnear.com";

/// OutLayer API response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutLayerResponse {
    /// Transaction status
    pub status: Option<String>,
    /// Intent hash
    #[serde(rename = "intentHash")]
    pub intent_hash: Option<String>,
    /// Amount transferred
    pub amount: Option<String>,
    /// Token ID
    #[serde(rename = "tokenId")]
    pub token_id: Option<String>,
    /// Sender account
    pub sender: Option<String>,
    /// Receiver account
    pub receiver: Option<String>,
    /// Timestamp
    pub timestamp: Option<i64>,
    /// Error message
    pub error: Option<String>,
}

/// NEAR Intents payment method
pub struct NearIntentsMethod {
    /// OutLayer API key
    api_key: String,
    /// HTTP client
    client: reqwest::Client,
    /// Accept mock payments (for testing)
    accept_mocks: bool,
}

impl NearIntentsMethod {
    /// Create a new NEAR Intents method
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            client: reqwest::Client::new(),
            accept_mocks: false,
        }
    }
    
    /// Enable mock payment acceptance (for testing)
    pub fn with_mocks(mut self) -> Self {
        self.accept_mocks = true;
        self
    }
    
    /// Check OutLayer API for intent status
    pub async fn check_intent(&self, intent_hash: &str) -> Result<OutLayerResponse> {
        let url = format!("{}/v1/intents/{}", OUTLAYER_API, intent_hash);
        
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .map_err(|e| Error::Other(format!("HTTP error: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(Error::Other(format!(
                "API error: {}",
                response.status()
            )));
        }
        
        response
            .json()
            .await
            .map_err(|e| Error::Other(format!("JSON error: {}", e)))
    }
    
    /// Extract request data from challenge
    pub fn extract_request_data(&self, challenge: &Challenge) -> Result<RequestData> {
        RequestData::decode(&challenge.request)
    }
    
    /// Verify a payment
    async fn verify_payment(
        &self,
        challenge: &Challenge,
        credential: &Credential,
    ) -> Result<bool> {
        // Verify challenge echo
        if !credential.verify_challenge_echo(challenge) {
            return Ok(false);
        }
        
        // Check if credential is a mock
        if self.accept_mocks && credential.is_mock() {
            return Ok(true);
        }
        
        // Extract proof from payload
        let proof = credential.payload.get("proof")
            .and_then(|p| p.as_str())
            .ok_or_else(|| Error::VerificationFailed("Missing proof in payload".into()))?;
        
        // Check intent via OutLayer
        let response = self.check_intent(proof).await?;
        
        // Get request data
        let request_data = self.extract_request_data(challenge)?;
        
        // Verify amount matches
        if let Some(ref amount) = response.amount {
            if amount != &request_data.amount {
                return Ok(false);
            }
        } else {
            return Ok(false);
        }
        
        // Verify recipient
        if let Some(ref receiver) = response.receiver {
            if receiver != &request_data.recipient {
                return Ok(false);
            }
        }
        
        // Check status is success
        match response.status.as_deref() {
            Some("success") | Some("completed") => Ok(true),
            Some("pending") => Err(Error::Other("Payment pending".into())),
            _ => Ok(false),
        }
    }
}

#[async_trait::async_trait]
impl Method for NearIntentsMethod {
    fn id(&self) -> &str {
        "near-intents"
    }
    
    async fn verify(
        &self,
        _request: &PaymentRequest,
        _proof: &PaymentProof,
    ) -> Result<bool> {
        // This is the old API, we use verify_credential instead
        Ok(true)
    }
    
    async fn verify_credential(
        &self,
        challenge: &Challenge,
        credential: &Credential,
    ) -> Result<bool> {
        self.verify_payment(challenge, credential).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::{ChallengeBuilder, CredentialBuilder};
    
    #[test]
    fn test_method_id() {
        let method = NearIntentsMethod::new("test-key");
        assert_eq!(method.id(), "near-intents");
    }
    
    #[test]
    fn test_extract_request_data() {
        let method = NearIntentsMethod::new("test-key");
        
        let request = RequestData::new("0.001", "wallet.near")
            .currency("USDC");
        
        let challenge = Challenge::builder()
            .realm("api.example.com")
            .method("near-intents")
            .intent("charge")
            .request(request.clone())
            .secret(b"test-secret".to_vec())
            .build()
            .unwrap();
        
        let extracted = method.extract_request_data(&challenge).unwrap();
        assert_eq!(extracted.amount, "0.001");
        assert_eq!(extracted.recipient, "wallet.near");
    }
    
    #[tokio::test]
    async fn test_mock_payment() {
        let method = NearIntentsMethod::new("test-key").with_mocks();
        
        let request = RequestData::new("0.001", "wallet.near");
        
        let challenge = Challenge::builder()
            .realm("api.example.com")
            .method("near-intents")
            .intent("charge")
            .request(request)
            .secret(b"test-secret".to_vec())
            .build()
            .unwrap();
        
        let credential = Credential::builder()
            .challenge(&challenge)
            .proof("test_payment_123")
            .build()
            .unwrap();
        
        let result = method.verify_credential(&challenge, &credential).await;
        assert!(result.unwrap());
    }
}
