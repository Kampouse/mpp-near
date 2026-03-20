//! MPP Challenge - Server-issued payment requirements
//!
//! A Challenge is returned with HTTP 402 responses to specify what payment
//! is required. It follows the MPP specification's Challenge format.

use serde::{Deserialize, Serialize};
use chrono::{Utc, DateTime};
use rand::Rng;
use base64::Engine;
use std::collections::HashMap;
use sha2::Sha256;
use hmac::{Hmac, Mac};

use crate::{Result, Error, VERSION, DEFAULT_CHALLENGE_TTL};

type HmacSha256 = Hmac<Sha256>;

/// Payment challenge returned with 402 responses
///
/// A Challenge specifies what payment is required to access a resource.
/// It includes the payment method, amount, recipient, and other details
/// needed for the client to complete the payment.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Challenge {
    /// Unique challenge identifier (required)
    /// Computed via HMAC-SHA256 binding for stateless verification
    #[serde(rename = "id")]
    pub id: String,
    
    /// Protection space identifier (required)
    #[serde(rename = "realm")]
    pub realm: String,
    
    /// Payment method identifier (required)
    /// Lowercase ASCII string identifying the payment network
    #[serde(rename = "method")]
    pub method: String,
    
    /// Payment intent (required)
    /// Common values: "charge" (one-time), "session" (streaming)
    #[serde(rename = "intent")]
    pub intent: String,
    
    /// Method-specific request data (required)
    /// Base64url-encoded JSON (JCS serialized, no padding)
    #[serde(rename = "request")]
    pub request: String,
    
    /// Challenge expiration timestamp (optional, recommended)
    /// RFC 3339 format: "2025-01-15T12:05:00Z"
    #[serde(skip_serializing_if = "Option::is_none", rename = "expires")]
    pub expires: Option<String>,
    
    /// Request body digest (optional)
    /// RFC 9530 format: "sha-256=:base64hash:"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    
    /// Human-readable description (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Server correlation data (optional)
    /// Base64url-encoded JSON (JCS serialized)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opaque: Option<String>,
}

/// Request data structure (encoded in challenge.request)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RequestData {
    /// Amount to pay (human-readable decimal string)
    pub amount: String,
    
    /// Token/currency symbol (e.g., "USDC", "NEAR")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    
    /// Token ID/contract address (optional)
    #[serde(rename = "tokenId", skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
    
    /// Recipient address
    pub recipient: String,
    
    /// Chain/network (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain: Option<String>,
    
    /// Method-specific details
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "methodDetails")]
    pub method_details: Option<serde_json::Value>,
    
    /// Additional fields
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl RequestData {
    /// Create new request data
    pub fn new(amount: impl Into<String>, recipient: impl Into<String>) -> Self {
        Self {
            amount: amount.into(),
            recipient: recipient.into(),
            currency: None,
            token_id: None,
            chain: None,
            method_details: None,
            extra: HashMap::new(),
        }
    }
    
    /// Set currency
    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = Some(currency.into());
        self
    }
    
    /// Set token ID
    pub fn token_id(mut self, token_id: impl Into<String>) -> Self {
        self.token_id = Some(token_id.into());
        self
    }
    
    /// Set chain
    pub fn chain(mut self, chain: impl Into<String>) -> Self {
        self.chain = Some(chain.into());
        self
    }
    
    /// Set method details
    pub fn method_details(mut self, details: serde_json::Value) -> Self {
        self.method_details = Some(details);
        self
    }
    
    /// Add extra field
    pub fn extra(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.extra.insert(key.into(), value);
        self
    }
    
    /// Encode to base64url (JCS serialized, no padding)
    pub fn encode(&self) -> Result<String> {
        // JCS canonical serialization
        let json = serde_json::to_string(self)?;
        Ok(base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(json))
    }
    
    /// Decode from base64url
    pub fn decode(encoded: &str) -> Result<Self> {
        let json_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(encoded)
            .map_err(|e| Error::InvalidChallenge(format!("Invalid base64: {}", e)))?;
        serde_json::from_slice(&json_bytes)
            .map_err(|e| Error::InvalidChallenge(format!("Invalid JSON: {}", e)))
    }
}

impl Challenge {
    /// Create a new challenge builder
    pub fn builder() -> ChallengeBuilder {
        ChallengeBuilder::new()
    }
    
    /// Check if this challenge is expired
    pub fn is_expired(&self) -> bool {
        if let Some(ref expires) = self.expires {
            if let Ok(exp) = DateTime::parse_from_rfc3339(expires) {
                return Utc::now() > exp.with_timezone(&Utc);
            }
        }
        false
    }
    
    /// Compute HMAC-SHA256 challenge ID for stateless verification
    pub fn compute_id(&self, secret: &[u8]) -> String {
        let input = self.binding_input();
        
        let mut mac = HmacSha256::new_from_slice(secret)
            .expect("HMAC can take key of any size");
        mac.update(input.as_bytes());
        
        base64::engine::general_purpose::URL_SAFE_NO_PAD
            .encode(mac.finalize().into_bytes())
    }
    
    /// Build the binding input string (7 positional slots)
    fn binding_input(&self) -> String {
        let slots: Vec<String> = vec![
            self.realm.clone(),
            self.method.clone(),
            self.intent.clone(),
            self.request.clone(),
            self.expires.clone().unwrap_or_default(),
            self.digest.clone().unwrap_or_default(),
            self.opaque.clone().unwrap_or_default(),
        ];
        slots.join("|")
    }
    
    /// Verify challenge ID matches binding
    pub fn verify_binding(&self, secret: &[u8]) -> bool {
        let expected = self.compute_id(secret);
        self.id == expected
    }
    
    /// Serialize to WWW-Authenticate header value
    pub fn to_www_authenticate(&self) -> String {
        let mut params = vec![
            format!(r#"id="{}""#, self.id),
            format!(r#"realm="{}""#, self.realm),
            format!(r#"method="{}""#, self.method),
            format!(r#"intent="{}""#, self.intent),
            format!(r#"request="{}""#, self.request),
        ];
        
        if let Some(ref expires) = self.expires {
            params.push(format!(r#"expires="{}""#, expires));
        }
        
        if let Some(ref digest) = self.digest {
            params.push(format!(r#"digest="{}""#, digest));
        }
        
        if let Some(ref description) = self.description {
            params.push(format!(r#"description="{}""#, description));
        }
        
        if let Some(ref opaque) = self.opaque {
            params.push(format!(r#"opaque="{}""#, opaque));
        }
        
        format!("Payment {}", params.join(", "))
    }
    
    /// Parse from WWW-Authenticate header
    pub fn from_www_authenticate(header: &str) -> Result<Self> {
        if !header.starts_with("Payment ") {
            return Err(Error::InvalidChallenge("Expected 'Payment ' prefix".into()));
        }
        
        let params = &header[8..];
        let mut id = None;
        let mut realm = None;
        let mut method = None;
        let mut intent = None;
        let mut request = None;
        let mut expires = None;
        let mut digest = None;
        let mut description = None;
        let mut opaque = None;
        
        // Parse key="value" pairs
        for part in params.split(',') {
            let part = part.trim();
            if let Some((key, value)) = part.split_once('=') {
                let value = value.trim_matches('"');
                match key.trim() {
                    "id" => id = Some(value.to_string()),
                    "realm" => realm = Some(value.to_string()),
                    "method" => method = Some(value.to_string()),
                    "intent" => intent = Some(value.to_string()),
                    "request" => request = Some(value.to_string()),
                    "expires" => expires = Some(value.to_string()),
                    "digest" => digest = Some(value.to_string()),
                    "description" => description = Some(value.to_string()),
                    "opaque" => opaque = Some(value.to_string()),
                    _ => {}
                }
            }
        }
        
        Ok(Challenge {
            id: id.ok_or_else(|| Error::InvalidChallenge("Missing id".into()))?,
            realm: realm.ok_or_else(|| Error::InvalidChallenge("Missing realm".into()))?,
            method: method.ok_or_else(|| Error::InvalidChallenge("Missing method".into()))?,
            intent: intent.ok_or_else(|| Error::InvalidChallenge("Missing intent".into()))?,
            request: request.ok_or_else(|| Error::InvalidChallenge("Missing request".into()))?,
            expires,
            digest,
            description,
            opaque,
        })
    }
}

/// Builder for creating challenges
#[derive(Debug)]
pub struct ChallengeBuilder {
    realm: Option<String>,
    method: Option<String>,
    intent: Option<String>,
    request: Option<RequestData>,
    request_encoded: Option<String>,
    expires: Option<String>,
    ttl: Option<i64>,
    digest: Option<String>,
    description: Option<String>,
    opaque: Option<String>,
    opaque_data: Option<HashMap<String, String>>,
    secret: Option<Vec<u8>>,
    id: Option<String>,
}

impl ChallengeBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            realm: None,
            method: None,
            intent: None,
            request: None,
            request_encoded: None,
            expires: None,
            ttl: None,
            digest: None,
            description: None,
            opaque: None,
            opaque_data: None,
            secret: None,
            id: None,
        }
    }
    
    /// Set realm (required)
    pub fn realm(mut self, realm: impl Into<String>) -> Self {
        self.realm = Some(realm.into());
        self
    }
    
    /// Set payment method (required)
    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }
    
    /// Set payment intent (required, default: "charge")
    pub fn intent(mut self, intent: impl Into<String>) -> Self {
        self.intent = Some(intent.into());
        self
    }
    
    /// Set challenge ID (computed via HMAC if not set)
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
    
    /// Set request data (will be encoded to base64url)
    pub fn request(mut self, request: RequestData) -> Self {
        self.request = Some(request);
        self
    }
    
    /// Set pre-encoded request (base64url JSON)
    pub fn request_encoded(mut self, encoded: impl Into<String>) -> Self {
        self.request_encoded = Some(encoded.into());
        self
    }
    
    /// Set expiration timestamp (RFC 3339)
    pub fn expires(mut self, expires: impl Into<String>) -> Self {
        self.expires = Some(expires.into());
        self
    }
    
    /// Set TTL in seconds (default: 300)
    pub fn ttl(mut self, ttl: i64) -> Self {
        self.ttl = Some(ttl);
        self
    }
    
    /// Set request body digest
    pub fn digest(mut self, digest: impl Into<String>) -> Self {
        self.digest = Some(digest.into());
        self
    }
    
    /// Set description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
    
    /// Set opaque data (will be encoded)
    pub fn opaque_data(mut self, data: HashMap<String, String>) -> Self {
        self.opaque_data = Some(data);
        self
    }
    
    /// Set pre-encoded opaque
    pub fn opaque(mut self, opaque: impl Into<String>) -> Self {
        self.opaque = Some(opaque.into());
        self
    }
    
    /// Set HMAC secret for stateless ID computation
    pub fn secret(mut self, secret: impl Into<Vec<u8>>) -> Self {
        self.secret = Some(secret.into());
        self
    }
    
    /// Build the challenge
    pub fn build(self) -> Result<Challenge> {
        let realm = self.realm.ok_or_else(|| 
            Error::InvalidChallenge("realm is required".into())
        )?;
        let method = self.method.ok_or_else(|| 
            Error::InvalidChallenge("method is required".into())
        )?;
        let intent = self.intent.unwrap_or_else(|| "charge".to_string());
        
        // Encode request
        let request = if let Some(encoded) = self.request_encoded {
            encoded
        } else if let Some(data) = self.request {
            data.encode()?
        } else {
            return Err(Error::InvalidChallenge("request is required".into()));
        };
        
        // Compute expires
        let expires = if let Some(exp) = self.expires {
            Some(exp)
        } else if let Some(ttl) = self.ttl {
            let exp = Utc::now() + chrono::Duration::seconds(ttl);
            Some(exp.to_rfc3339())
        } else {
            let exp = Utc::now() + chrono::Duration::seconds(DEFAULT_CHALLENGE_TTL);
            Some(exp.to_rfc3339())
        };
        
        // Encode opaque
        let opaque = if let Some(encoded) = self.opaque {
            Some(encoded)
        } else if let Some(data) = self.opaque_data {
            let json = serde_json::to_string(&data)?;
            Some(base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(json))
        } else {
            None
        };
        
        let challenge = Challenge {
            id: String::new(), // Will be computed
            realm,
            method,
            intent,
            request,
            expires,
            digest: self.digest,
            description: self.description,
            opaque,
        };
        
        // Compute ID
        let id = if let Some(custom_id) = self.id {
            custom_id
        } else if let Some(ref secret) = self.secret {
            challenge.compute_id(secret)
        } else {
            // Generate random ID if no secret provided
            hex::encode(rand::thread_rng().gen::<[u8; 16]>())
        };
        
        Ok(Challenge { id, ..challenge })
    }
}

impl Default for ChallengeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_request_data_encoding() {
        let req = RequestData::new("1000", "acct_123")
            .currency("usd");
        
        let encoded = req.encode().unwrap();
        let decoded = RequestData::decode(&encoded).unwrap();
        
        assert_eq!(decoded.amount, "1000");
        assert_eq!(decoded.recipient, "acct_123");
        assert_eq!(decoded.currency, Some("usd".to_string()));
    }
    
    #[test]
    fn test_challenge_builder() {
        let req = RequestData::new("0.001", "wallet.near")
            .currency("USDC");
        
        let challenge = Challenge::builder()
            .realm("api.example.com")
            .method("near-intents")
            .intent("charge")
            .request(req)
            .description("API access")
            .build()
            .unwrap();
        
        assert_eq!(challenge.method, "near-intents");
        assert_eq!(challenge.intent, "charge");
        assert!(challenge.expires.is_some());
        assert!(!challenge.is_expired());
    }
    
    #[test]
    fn test_hmac_binding() {
        let secret = b"test-secret-key";
        let req = RequestData::new("1000", "acct_123");
        
        let challenge = Challenge::builder()
            .realm("api.example.com")
            .method("test")
            .intent("charge")
            .request(req)
            .secret(secret.to_vec())
            .build()
            .unwrap();
        
        // Verify binding
        assert!(challenge.verify_binding(secret));
        assert!(!challenge.verify_binding(b"wrong-secret"));
    }
    
    #[test]
    fn test_www_authenticate_roundtrip() {
        let req = RequestData::new("1000", "acct_123");
        
        let original = Challenge::builder()
            .realm("api.example.com")
            .method("test")
            .intent("charge")
            .request(req)
            .expires("2025-01-15T12:05:00Z")
            .id("test-id-123")
            .build()
            .unwrap();
        
        let header = original.to_www_authenticate();
        let parsed = Challenge::from_www_authenticate(&header).unwrap();
        
        assert_eq!(parsed.id, original.id);
        assert_eq!(parsed.realm, original.realm);
        assert_eq!(parsed.method, original.method);
        assert_eq!(parsed.request, original.request);
    }
}
