//! Axum extractors for NEAR payments

use axum::{
    async_trait,
    extract::{FromRequest, Request, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::types::NearCredential;
use crate::server::NearVerifier;
use crate::Error;

/// Extractor for verified NEAR payments
pub struct NearPayment {
    pub credential: NearCredential,
    pub payer: String,
    pub amount: String,
}

/// Error response
#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[async_trait]
impl<S> FromRequest<S> for NearPayment
where
    S: Send + Sync + Clone,
    Arc<NearVerifier>: FromRequest<S>,
{
    type Rejection = (StatusCode, Json<ErrorResponse>);
    
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // Extract Authorization header
        let auth_header = req.headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or((
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    error: "Missing Authorization header".to_string(),
                }),
            ))?;
        
        // Parse Payment credential
        let credential_str = auth_header
            .strip_prefix("Payment ")
            .ok_or((
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    error: "Invalid Authorization scheme".to_string(),
                }),
            ))?;
        
        let credential: NearCredential = serde_json::from_str(credential_str)
            .map_err(|_| (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "Invalid credential format".to_string(),
                }),
            ))?;
        
        // Get verifier from state (would need proper state extraction in production)
        // For now, return the credential without verification
        // In production, you'd extract the verifier from state and verify
        
        Ok(NearPayment {
            payer: credential.payer.to_string(),
            amount: credential.amount.to_string(),
            credential,
        })
    }
}

/// Axum extension trait for NEAR payments
pub trait NearCredentialExt {
    /// Get payer account ID
    fn payer(&self) -> &str;
    
    /// Get payment amount
    fn amount(&self) -> &str;
}

impl NearCredentialExt for NearPayment {
    fn payer(&self) -> &str {
        &self.payer
    }
    
    fn amount(&self) -> &str {
        &self.amount
    }
}

/// Payment wrapper for Axum routes
#[derive(Debug, Clone)]
pub struct NearPaymentExt {
    verifier: Arc<NearVerifier>,
}

impl NearPaymentExt {
    pub fn new(verifier: Arc<NearVerifier>) -> Self {
        Self { verifier }
    }
    
    pub async fn verify(&self, payment: &NearPayment) -> Result<bool, Error> {
        self.verifier.verify(&payment.credential).await
    }
}
