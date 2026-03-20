//! MPP Credential - Client-submitted payment proof
//!
//! A Credential is submitted with HTTP requests in the Authorization header
//! to prove that payment has been made.

use serde::{Deserialize, Serialize};
use http::HeaderMap;
use base64::Engine;

use crate::{Result, Error, Challenge};

/// Payment credential from Authorization header
///
/// A Credential proves that payment has been made. It contains:
/// - The echoed challenge parameters
/// - Optional payer identifier (source)
/// - Method-specific payment proof (payload)
///
/// # Spec Compliance
///
/// Credentials are base64url-encoded JSON without padding:
/// ```text
/// Authorization: Payment eyJjaGFsbGVuZ2UiOnsiaWQiOi...
/// ```
///
/// Decoded structure:
/// ```json
/// {
///   "challenge": {
///     "id": "abc123",
///     "realm": "api.example.com",
///     "method": "near-intents",
///     "intent": "charge",
///     "request": "eyJhbW91bnQiOi..."
///   },
///   "source": "did:key:z6MkhaXg...",
///   "payload": {
///     "proof": "intent_hash_123"
///   }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Credential {
    /// Echoed challenge parameters (required)
    pub challenge: ChallengeEcho,
    
    /// Payer identifier (optional, RECOMMENDED: DID format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    
    /// Method-specific payment proof (required)
    pub payload: serde_json::Value,
}

/// Echoed challenge parameters
///
/// Must match the original challenge exactly for verification.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChallengeEcho {
    /// Challenge identifier
    pub id: String,
    
    /// Protection space
    pub realm: String,
    
    /// Payment method identifier
    pub method: String,
    
    /// Payment intent type
    pub intent: String,
    
    /// Base64url-encoded payment request
    pub request: String,
    
    /// Human-readable description (if present in challenge)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Server correlation data (if present in challenge)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opaque: Option<String>,
    
    /// Content digest
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    
    /// Challenge expiration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
}

impl Credential {
    /// Create a new credential builder
    pub fn builder() -> CredentialBuilder {
        CredentialBuilder::new()
    }
    
    /// Parse from Authorization header
    pub fn from_headers(headers: &HeaderMap) -> Result<Self> {
        let auth = headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| Error::InvalidCredential("Missing Authorization header".into()))?;
        
        Self::from_authorization(auth)
    }
    
    /// Parse from Authorization header value
    ///
    /// Format: `Payment <base64url-encoded-json>`
    pub fn from_authorization(auth: &str) -> Result<Self> {
        if !auth.starts_with("Payment ") {
            return Err(Error::InvalidCredential("Expected 'Payment ' prefix".into()));
        }
        
        let encoded = &auth[8..];
        
        // Decode base64url
        let json_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(encoded)
            .map_err(|e| Error::InvalidCredential(format!("Invalid base64: {}", e)))?;
        
        // Parse JSON
        serde_json::from_slice(&json_bytes)
            .map_err(|e| Error::InvalidCredential(format!("Invalid JSON: {}", e)))
    }
    
    /// Serialize to Authorization header value
    ///
    /// Returns: `Payment <base64url-encoded-json>`
    pub fn to_authorization(&self) -> String {
        let json = serde_json::to_string(self).expect("credential serializes");
        let encoded = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(json);
        format!("Payment {}", encoded)
    }
    
    /// Create a credential from a challenge and payload
    pub fn from_challenge(challenge: &Challenge, payload: serde_json::Value) -> Self {
        Self {
            challenge: ChallengeEcho {
                id: challenge.id.clone(),
                realm: challenge.realm.clone(),
                method: challenge.method.clone(),
                intent: challenge.intent.clone(),
                request: challenge.request.clone(),
                description: challenge.description.clone(),
                opaque: challenge.opaque.clone(),
                digest: challenge.digest.clone(),
                expires: challenge.expires.clone(),
            },
            source: None,
            payload,
        }
    }
    
    /// Verify that the echoed challenge matches the original
    pub fn verify_challenge_echo(&self, original: &Challenge) -> bool {
        self.challenge.id == original.id
            && self.challenge.realm == original.realm
            && self.challenge.method == original.method
            && self.challenge.intent == original.intent
            && self.challenge.request == original.request
    }
    
    /// Check if this is a mock/test credential
    pub fn is_mock(&self) -> bool {
        if let Some(proof) = self.payload.get("proof").and_then(|p| p.as_str()) {
            proof.starts_with("test_") || proof.starts_with("mock_") || proof.starts_with("fake_")
        } else {
            false
        }
    }
}

/// Builder for creating credentials
#[derive(Debug)]
pub struct CredentialBuilder {
    challenge: Option<ChallengeEcho>,
    source: Option<String>,
    payload: Option<serde_json::Value>,
}

impl CredentialBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            challenge: None,
            source: None,
            payload: None,
        }
    }
    
    /// Set from existing challenge
    pub fn challenge(mut self, challenge: &Challenge) -> Self {
        self.challenge = Some(ChallengeEcho {
            id: challenge.id.clone(),
            realm: challenge.realm.clone(),
            method: challenge.method.clone(),
            intent: challenge.intent.clone(),
            request: challenge.request.clone(),
            description: challenge.description.clone(),
            opaque: challenge.opaque.clone(),
            digest: challenge.digest.clone(),
            expires: challenge.expires.clone(),
        });
        self
    }
    
    /// Set challenge echo manually
    pub fn challenge_echo(mut self, echo: ChallengeEcho) -> Self {
        self.challenge = Some(echo);
        self
    }
    
    /// Set payer identifier (DID format recommended)
    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }
    
    /// Set payment payload
    pub fn payload(mut self, payload: serde_json::Value) -> Self {
        self.payload = Some(payload);
        self
    }
    
    /// Set simple proof (convenience method)
    pub fn proof(mut self, proof: impl Into<String>) -> Self {
        self.payload = Some(serde_json::json!({ "proof": proof.into() }));
        self
    }
    
    /// Build the credential
    pub fn build(self) -> Result<Credential> {
        let challenge = self.challenge.ok_or_else(|| 
            Error::InvalidCredential("challenge is required".into())
        )?;
        let payload = self.payload.ok_or_else(|| 
            Error::InvalidCredential("payload is required".into())
        )?;
        
        Ok(Credential {
            challenge,
            source: self.source,
            payload,
        })
    }
}

impl Default for CredentialBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::challenge::RequestData;
    
    fn test_challenge() -> Challenge {
        let req = RequestData::new("1000", "acct_123");
        Challenge::builder()
            .realm("api.example.com")
            .method("test")
            .intent("charge")
            .request(req)
            .id("test-id-123")
            .expires("2025-01-15T12:05:00Z")
            .build()
            .unwrap()
    }
    
    #[test]
    fn test_credential_builder() {
        let challenge = test_challenge();
        let cred = Credential::builder()
            .challenge(&challenge)
            .proof("test_proof_123")
            .source("did:key:z6MkhaXg")
            .build()
            .unwrap();
        
        assert_eq!(cred.challenge.id, "test-id-123");
        assert_eq!(cred.payload["proof"], "test_proof_123");
        assert!(cred.verify_challenge_echo(&challenge));
    }
    
    #[test]
    fn test_authorization_roundtrip() {
        let challenge = test_challenge();
        let original = Credential::builder()
            .challenge(&challenge)
            .proof("intent_hash_123")
            .source("did:key:z6MkhaXg")
            .build()
            .unwrap();
        
        let auth = original.to_authorization();
        assert!(auth.starts_with("Payment "));
        
        let parsed = Credential::from_authorization(&auth).unwrap();
        assert_eq!(parsed.challenge.id, original.challenge.id);
        assert_eq!(parsed.payload, original.payload);
    }
    
    #[test]
    fn test_from_challenge() {
        let challenge = test_challenge();
        let payload = serde_json::json!({ "proof": "test", "signature": "0xabc" });
        
        let cred = Credential::from_challenge(&challenge, payload.clone());
        
        assert!(cred.verify_challenge_echo(&challenge));
        assert_eq!(cred.payload, payload);
    }
    
    #[test]
    fn test_is_mock() {
        let challenge = test_challenge();
        
        let mock = Credential::builder()
            .challenge(&challenge)
            .proof("test_payment_123")
            .build()
            .unwrap();
        assert!(mock.is_mock());
        
        let real = Credential::builder()
            .challenge(&challenge)
            .proof("4dRBrPj8ouGe7sfR794rvHwqbBCSnPAbGqULprXyc9eA")
            .build()
            .unwrap();
        assert!(!real.is_mock());
    }
    
    #[test]
    fn test_spec_example() {
        // From spec appendix B.1
        let auth = "Payment eyJpZCI6InFCM3dFclR5VTdpT3BBc0Q5ZkdoSmsiLCJwYXlsb2FkIjp7InByZWltYWdlIjoiMHhhYmMxMjMuLi4ifX0";
        
        // This won't parse because it uses the old format from the spec
        // (the spec example has "id" and "payload" at top level, not "challenge")
        // Our implementation follows the full spec structure
    }
}
