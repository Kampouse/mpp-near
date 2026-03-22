//! Bridge server for cross-chain MPP payments

use crate::bridge::types::{BridgeError, BridgeRequest, BridgeResponse, BridgeStatus, Chain, TokenInfo};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

/// Bridge server configuration
#[derive(Debug, Clone)]
pub struct BridgeConfig {
    /// OutLayer API key
    pub outlayer_api_key: String,
    
    /// OutLayer API URL
    pub outlayer_url: String,
    
    /// Server listen address
    pub listen_addr: SocketAddr,
    
    /// Payment verification timeout (ms)
    pub verification_timeout_ms: u64,
    
    /// Bridge fee (percentage, e.g., 0.1 = 0.1%)
    pub bridge_fee_percent: f64,
}

impl Default for BridgeConfig {
    fn default() -> Self {
        Self {
            outlayer_api_key: String::new(),
            outlayer_url: "https://api.outlayer.fastnear.com".into(),
            listen_addr: "0.0.0.0:3001".parse().unwrap(),
            verification_timeout_ms: 60_000,
            bridge_fee_percent: 0.0, // No fee by default
        }
    }
}

/// Bridge server state
#[derive(Debug, Clone)]
pub struct BridgeServer {
    config: BridgeConfig,
    http: Client,
    payments: Arc<RwLock<HashMap<String, BridgeResponse>>>,
}

impl BridgeServer {
    /// Create a new bridge server
    pub fn new(config: BridgeConfig) -> Self {
        Self {
            http: Client::builder()
                .timeout(Duration::from_millis(config.verification_timeout_ms))
                .build()
                .unwrap_or_default(),
            config,
            payments: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Build Axum router
    pub fn router(&self) -> Router {
        Router::new()
            .route("/bridge", post(handle_bridge))
            .route("/bridge/status/:tx", get(handle_status))
            .route("/health", get(handle_health))
            .route("/chains", get(handle_chains))
            .route("/tokens", get(handle_tokens))
            .with_state(Arc::new(self.clone()))
    }
    
    /// Start the server
    pub async fn serve(self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = self.config.listen_addr;
        let app = self.router();
        
        tracing::info!("🌉 MPP Bridge listening on {}", addr);
        
        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;
        
        Ok(())
    }
    
    /// Process bridge payment
    async fn process_payment(&self, request: BridgeRequest) -> Result<BridgeResponse, BridgeError> {
        let token_info = TokenInfo::from_symbol(&request.token)
            .ok_or_else(|| BridgeError {
                error: "invalid_token".into(),
                message: format!("Unsupported token: {}", request.token),
                code: Some(400),
            })?;
        
        // Add bridge fee
        let fee_amount = request.amount * self.config.bridge_fee_percent / 100.0;
        let total_amount = request.amount + fee_amount;
        
        // Convert to raw amount
        let amount_raw = (total_amount * 10f64.powi(token_info.decimals as i32)) as u64;
        
        // Call OutLayer cross-chain withdraw
        let url = format!("{}/wallet/v1/intents/withdraw", self.config.outlayer_url);
        
        let withdraw_request = serde_json::json!({
            "to": request.recipient,
            "amount": amount_raw.to_string(),
            "token": token_info.near_token_id,
            "chain": request.target_chain,
        });
        
        let resp = self.http
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.outlayer_api_key))
            .json(&withdraw_request)
            .send()
            .await
            .map_err(|e| BridgeError {
                error: "http_error".into(),
                message: format!("OutLayer request failed: {}", e),
                code: None,
            })?;
        
        if !resp.status().is_success() {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".into());
            return Err(BridgeError {
                error: "withdraw_failed".into(),
                message: error_text,
                code: None,
            });
        }
        
        let withdraw_response: OutLayerWithdrawResponse = resp.json().await.map_err(|e| BridgeError {
            error: "parse_error".into(),
            message: e.to_string(),
            code: None,
        })?;
        
        let response = BridgeResponse {
            near_tx: withdraw_response.near_tx.unwrap_or_default(),
            target_tx: withdraw_response.target_tx.clone(),
            target_chain: request.target_chain.clone(),
            status: BridgeStatus::Submitted,
            amount: request.amount,
            token: request.token.clone(),
            timestamp: chrono::Utc::now().timestamp(),
            estimated_confirmation_ms: Some(30_000),
        };
        
        // Store for status queries
        {
            let mut payments = self.payments.write().await;
            payments.insert(withdraw_response.target_tx.clone(), response.clone());
        }
        
        Ok(response)
    }
    
    /// Get payment status
    async fn get_status(&self, tx_hash: &str) -> Option<BridgeResponse> {
        let payments = self.payments.read().await;
        payments.get(tx_hash).cloned()
    }
}

/// OutLayer withdraw API response
#[derive(Debug, Deserialize)]
struct OutLayerWithdrawResponse {
    near_tx: Option<String>,
    target_tx: String,
    status: String,
}

// HTTP Handlers

async fn handle_bridge(
    State(server): State<Arc<BridgeServer>>,
    Json(request): Json<BridgeRequest>,
) -> Result<Json<BridgeResponse>, BridgeError> {
    // Validate chain
    let chain = Chain::from_str(&request.target_chain)
        .ok_or_else(|| BridgeError {
            error: "invalid_chain".into(),
            message: format!("Unsupported chain: {}", request.target_chain),
            code: Some(400),
        })?;
    
    // Validate address
    if !chain.is_valid_address(&request.recipient) {
        return Err(BridgeError {
            error: "invalid_address".into(),
            message: format!("Invalid {} address format", chain.as_str()),
            code: Some(400),
        });
    }
    
    // Process payment
    let response = server.process_payment(request).await?;
    
    Ok(Json(response))
}

async fn handle_status(
    State(server): State<Arc<BridgeServer>>,
    axum::extract::Path(tx_hash): axum::extract::Path<String>,
) -> Result<Json<BridgeResponse>, BridgeError> {
    server.get_status(&tx_hash).await
        .map(Json)
        .ok_or_else(|| BridgeError {
            error: "not_found".into(),
            message: format!("Payment not found: {}", tx_hash),
            code: Some(404),
        })
}

async fn handle_health() -> &'static str {
    "OK"
}

async fn handle_chains() -> Json<Vec<&'static str>> {
    Json(vec![
        "near", "ethereum", "solana", "bitcoin", "arbitrum", "base",
        "polygon", "optimism", "avalanche", "bsc", "ton", "aptos",
        "sui", "starknet", "tron", "stellar", "dogecoin", "xrp",
        "zcash", "litecoin",
    ])
}

async fn handle_tokens() -> Json<Vec<TokenInfo>> {
    Json(vec![
        TokenInfo::from_symbol("usdc").unwrap(),
        TokenInfo::from_symbol("usdt").unwrap(),
        TokenInfo::from_symbol("btc").unwrap(),
        TokenInfo::from_symbol("eth").unwrap(),
        TokenInfo::from_symbol("sol").unwrap(),
    ])
}

// Error handling
impl IntoResponse for BridgeError {
    fn into_response(self) -> Response {
        let code = self.code.unwrap_or(500);
        (StatusCode::from_u16(code as u16).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR), Json(self)).into_response()
    }
}

/// Run bridge server (convenience function)
pub async fn run_bridge(config: BridgeConfig) -> Result<(), Box<dyn std::error::Error>> {
    let server = BridgeServer::new(config);
    server.serve().await
}
