//! MPP Method - Trait for implementing payment methods
//!
//! Payment methods define how specific payment networks integrate with MPP.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use async_trait::async_trait;

use crate::{Challenge, Credential, Error, Result, RequestData};

/// Payment request details (from challenge.request)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentRequest {
    /// Amount to pay
    pub amount: String,
    /// Currency/token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Token ID/contract (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    /// Recipient address
    pub recipient: String,
    /// Chain/network (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain: Option<String>,
    /// Challenge ID
    pub challenge_id: String,
    /// Challenge realm
    pub realm: String,
    /// Payment method
    pub method: String,
    /// Payment intent
    pub intent: String,
}

/// Payment proof (from credential.payload)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentProof {
    /// Proof/transaction hash
    pub proof: String,
    /// Account that paid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Signature (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    /// Public key (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    /// Additional fields
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl PaymentProof {
    /// Create a new payment proof
    pub fn new(proof: impl Into<String>) -> Self {
        Self {
            proof: proof.into(),
            account: None,
            signature: None,
            public_key: None,
            extra: HashMap::new(),
        }
    }
    
    /// Parse from credential payload JSON
    pub fn from_payload(payload: &serde_json::Value) -> Result<Self> {
        let proof = payload.get("proof")
            .and_then(|p| p.as_str())
            .ok_or_else(|| Error::InvalidCredential("Missing proof".into()))?
            .to_string();
        
        Ok(Self {
            proof,
            account: payload.get("account").and_then(|v| v.as_str()).map(|s| s.to_string()),
            signature: payload.get("signature").and_then(|v| v.as_str()).map(|s| s.to_string()),
            public_key: payload.get("publicKey").and_then(|v| v.as_str()).map(|s| s.to_string()),
            extra: HashMap::new(),
        })
    }
}

/// Payment method trait
///
/// Implement this trait to add support for a payment network.
#[async_trait]
pub trait Method: Send + Sync {
    /// Get the method identifier (lowercase ASCII)
    fn id(&self) -> &str;
    
    /// Build a challenge for this method
    fn build_challenge(&self, request: &PaymentRequest, secret: &[u8]) -> Result<Challenge> {
        let req_data = RequestData::new(&request.amount, &request.recipient);
        
        Challenge::builder()
            .realm(&request.realm)
            .method(self.id())
            .intent(&request.intent)
            .request(req_data)
            .secret(secret.to_vec())
            .build()
    }
    
    /// Verify a payment proof
    async fn verify(
        &self,
        request: &PaymentRequest,
        proof: &PaymentProof,
    ) -> Result<bool>;
    
    /// Extract payment request from challenge
    fn extract_request(&self, challenge: &Challenge) -> Result<PaymentRequest> {
        let request_data = RequestData::decode(&challenge.request)?;
        
        Ok(PaymentRequest {
            amount: request_data.amount,
            currency: request_data.currency,
            token_id: request_data.token_id,
            recipient: request_data.recipient,
            chain: request_data.chain,
            challenge_id: challenge.id.clone(),
            realm: challenge.realm.clone(),
            method: challenge.method.clone(),
            intent: challenge.intent.clone(),
        })
    }
    
    /// Extract payment proof from credential
    fn extract_proof(&self, credential: &Credential) -> Result<PaymentProof> {
        PaymentProof::from_payload(&credential.payload)
    }
    
    /// Verify credential against challenge
    async fn verify_credential(
        &self,
        challenge: &Challenge,
        credential: &Credential,
    ) -> Result<bool> {
        // Verify challenge echo
        if !credential.verify_challenge_echo(challenge) {
            return Ok(false);
        }
        
        let request = self.extract_request(challenge)?;
        let proof = self.extract_proof(credential)?;
        self.verify(&request, &proof).await
    }
}

/// Registry of payment methods
pub struct MethodRegistry {
    methods: HashMap<String, Box<dyn Method>>,
}

impl MethodRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            methods: HashMap::new(),
        }
    }
    
    /// Register a payment method
    pub fn register<M: Method + 'static>(&mut self, method: M) {
        self.methods.insert(method.id().to_string(), Box::new(method));
    }
    
    /// Get a method by ID
    pub fn get(&self, id: &str) -> Option<&dyn Method> {
        self.methods.get(id).map(|m| m.as_ref())
    }
    
    /// Check if a method is registered
    pub fn contains(&self, id: &str) -> bool {
        self.methods.contains_key(id)
    }
    
    /// List all registered method IDs
    pub fn list(&self) -> Vec<&str> {
        self.methods.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for MethodRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    struct TestMethod;
    
    #[async_trait]
    impl Method for TestMethod {
        fn id(&self) -> &str {
            "test"
        }
        
        async fn verify(&self, _request: &PaymentRequest, _proof: &PaymentProof) -> Result<bool> {
            Ok(true)
        }
    }
    
    #[test]
    fn test_method_registry() {
        let mut registry = MethodRegistry::new();
        registry.register(TestMethod);
        
        assert!(registry.contains("test"));
        assert!(registry.get("test").is_some());
        assert_eq!(registry.list(), vec!["test"]);
    }
    
    #[tokio::test]
    async fn test_verify_credential() {
        let method = TestMethod;
        let request = RequestData::new("1.0", "wallet.near");
        
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
        
        let result = method.verify_credential(&challenge, &credential).await;
        assert!(result.unwrap());
    }
}
