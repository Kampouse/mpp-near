//! MPP Axum Middleware
//!
//! Provides middleware and extractors for payment-gated routes.

use axum::{
    http::{HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

use crate::primitives::{
    Challenge, RequestData, Credential, Receipt, Problem,
    Verifier,
    WWW_AUTHENTICATE, PAYMENT_RECEIPT, CACHE_CONTROL, CACHE_NO_STORE, CACHE_PRIVATE,
};

/// Payment configuration
#[derive(Debug, Clone)]
pub struct PaymentConfig {
    /// Realm (protection space)
    pub realm: String,
    /// Payment method to use
    pub method: String,
    /// Default recipient address
    pub recipient: String,
    /// Default token
    pub default_token: String,
    /// Challenge TTL in seconds
    pub challenge_ttl: i64,
    /// HMAC secret for stateless verification
    pub secret: Vec<u8>,
}

impl Default for PaymentConfig {
    fn default() -> Self {
        Self {
            realm: "api.example.com".to_string(),
            method: "near-intents".to_string(),
            recipient: String::new(),
            default_token: "USDC".to_string(),
            challenge_ttl: 300,
            secret: Vec::new(),
        }
    }
}

/// Pricing for an endpoint
#[derive(Debug, Clone)]
pub struct Pricing {
    /// Amount to charge
    pub amount: String,
    /// Token/currency
    pub token: Option<String>,
    /// Token ID/contract
    pub token_id: Option<String>,
    /// Human-readable description
    pub description: Option<String>,
}

/// Payment middleware state
pub struct PaymentState {
    /// Payment method verifier
    pub verifier: Arc<dyn Verifier>,
    /// Configuration
    pub config: PaymentConfig,
    /// Endpoint pricing
    pub pricing: HashMap<String, Pricing>,
    /// Active challenges (for stateful mode, if no secret)
    pub challenges: Arc<RwLock<HashMap<String, Challenge>>>,
}

impl PaymentState {
    /// Create new payment state
    pub fn new<V: Verifier + 'static>(verifier: V, config: PaymentConfig) -> Self {
        Self {
            verifier: Arc::new(verifier),
            config,
            pricing: HashMap::new(),
            challenges: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Add pricing for an endpoint
    pub fn with_pricing(mut self, path: &str, amount: &str) -> Self {
        self.pricing.insert(
            path.to_string(),
            Pricing {
                amount: amount.to_string(),
                token: None,
                token_id: None,
                description: None,
            },
        );
        self
    }
    
    /// Add pricing with token
    pub fn with_pricing_token(mut self, path: &str, amount: &str, token: &str) -> Self {
        self.pricing.insert(
            path.to_string(),
            Pricing {
                amount: amount.to_string(),
                token: Some(token.to_string()),
                token_id: None,
                description: None,
            },
        );
        self
    }
    
    /// Add pricing with description
    pub fn with_pricing_desc(
        mut self,
        path: &str,
        amount: &str,
        token: &str,
        description: &str,
    ) -> Self {
        self.pricing.insert(
            path.to_string(),
            Pricing {
                amount: amount.to_string(),
                token: Some(token.to_string()),
                token_id: None,
                description: Some(description.to_string()),
            },
        );
        self
    }
    
    /// Get pricing for an endpoint
    pub fn get_pricing(&self, path: &str) -> Option<&Pricing> {
        self.pricing.get(path)
    }
}

/// Payment layer wrapper
pub struct PaymentLayer {
    state: Arc<PaymentState>,
}

impl PaymentLayer {
    /// Create new payment layer
    pub fn new<V: Verifier + 'static>(verifier: V, config: PaymentConfig) -> Self {
        Self {
            state: Arc::new(PaymentState::new(verifier, config)),
        }
    }
    
    /// Create from existing state
    pub fn from_state(state: PaymentState) -> Self {
        Self {
            state: Arc::new(state),
        }
    }
    
    /// Get the state
    pub fn state(&self) -> Arc<PaymentState> {
        self.state.clone()
    }
}

/// Extractor for verified payment
pub struct Paid {
    /// Challenge that was paid
    pub challenge: Challenge,
    /// Credential that was used
    pub credential: Credential,
    /// Receipt for the payment
    pub receipt: Receipt,
}

/// Create a 402 challenge response
pub async fn create_challenge_response(
    config: &PaymentConfig,
    pricing: &Pricing,
) -> Response {
    let token = pricing.token.as_ref().unwrap_or(&config.default_token);
    
    // Create request data
    let request = RequestData::new(&pricing.amount, &config.recipient)
        .currency(token);
    
    let request = if let Some(ref token_id) = pricing.token_id {
        request.token_id(token_id)
    } else {
        request
    };
    
    // Create challenge
    let mut builder = Challenge::builder()
        .realm(&config.realm)
        .method(&config.method)
        .intent("charge")
        .request(request)
        .ttl(config.challenge_ttl)
        .secret(config.secret.clone());
    
    if let Some(ref desc) = pricing.description {
        builder = builder.description(desc);
    }
    
    let challenge = builder.build().expect("valid challenge");

    // Build response
    let www_auth = HeaderValue::from_str(&challenge.to_www_authenticate())
        .unwrap_or_else(|_| HeaderValue::from_static("Payment"));

    (
        StatusCode::PAYMENT_REQUIRED,
        [(WWW_AUTHENTICATE, www_auth)],
        [(CACHE_CONTROL, HeaderValue::from_static(CACHE_NO_STORE))],
        axum::Json(serde_json::json!({
            "error": "Payment Required",
            "challenge": {
                "id": challenge.id,
                "realm": challenge.realm,
                "method": challenge.method,
                "intent": challenge.intent,
                "request": challenge.request,
                "expires": challenge.expires,
            },
        })),
    )
        .into_response()
}

/// Create a 402 error response with problem
pub fn create_problem_response(problem: Problem) -> Response {
    (
        StatusCode::PAYMENT_REQUIRED,
        [(CACHE_CONTROL, HeaderValue::from_static(CACHE_NO_STORE))],
        axum::Json(problem),
    )
        .into_response()
}

/// Add receipt header to response
pub fn add_receipt_header(response: &mut Response, receipt: &Receipt) {
    if let Ok(value) = HeaderValue::from_str(&receipt.to_header()) {
        response.headers_mut().insert(PAYMENT_RECEIPT, value);
    }
    let value = HeaderValue::from_static(CACHE_PRIVATE);
    response.headers_mut().insert(CACHE_CONTROL, value);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::{ChallengeBuilder, CredentialBuilder};
    
    #[test]
    fn test_payment_config() {
        let config = PaymentConfig {
            realm: "api.example.com".to_string(),
            recipient: "wallet.near".to_string(),
            secret: b"test-secret".to_vec(),
            ..Default::default()
        };
        
        assert_eq!(config.method, "near-intents");
        assert_eq!(config.challenge_ttl, 300);
    }
    
    #[test]
    fn test_payment_state_builder() {
        struct MockVerifier;
        
        #[async_trait::async_trait]
        impl Verifier for MockVerifier {
            async fn verify(
                &self,
                _challenge: &Challenge,
                _credential: &Credential,
            ) -> crate::primitives::Result<crate::primitives::VerificationResult> {
                Ok(crate::primitives::VerificationResult::valid("1.0", "TEST"))
            }
        }
        
        let config = PaymentConfig {
            realm: "api.example.com".to_string(),
            recipient: "wallet.near".to_string(),
            secret: b"test-secret".to_vec(),
            ..Default::default()
        };
        
        let state = PaymentState::new(MockVerifier, config)
            .with_pricing("/api/test", "0.001")
            .with_pricing_token("/api/search", "0.001", "USDC")
            .with_pricing_desc("/api/premium", "0.01", "USDC", "Premium API");
        
        assert!(state.get_pricing("/api/test").is_some());
        assert!(state.get_pricing("/api/premium").is_some());
    }
}
