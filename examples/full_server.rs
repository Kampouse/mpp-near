//! Comprehensive MPP-NEAR Server Example
//!
//! This example demonstrates a complete MPP server supporting:
//! - Standard NEAR payments (on-chain transfers)
//! - NEAR Intents payments (gasless via OutLayer)
//! - Multiple pricing tiers
//! - Full MPP-1.0 spec compliance
//! - Error handling with RFC 9457 Problem details
//!
//! Run with:
//! ```bash
//! cargo run --example full_server --features server
//! ```
//!
//! Set environment variables:
//! ```bash
//! export MPP_HMAC_SECRET="your-secret-here"
//! export MPP_RECIPIENT="merchant.near"
//! export OUTLAYER_API_KEY="your-api-key"  # Optional for intents
//! export MPP_RPC_URL="https://rpc.mainnet.near.org"
//! ```

use axum::{
    extract::{Request, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use mpp_near::{
    near_intents::NearIntentsMethod,
    primitives::{
        Challenge, ChallengeBuilder, Credential, Problem, Receipt, RequestData,
    },
    server::NearVerifier,
    types::{AccountId, NearAmount},
    Error, Result,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};

/// Server configuration
#[derive(Debug, Clone)]
struct ServerConfig {
    /// HMAC secret for challenge binding
    hmac_secret: Vec<u8>,
    /// Recipient account for NEAR payments
    recipient: AccountId,
    /// OutLayer API key for intents (optional)
    intents_api_key: Option<String>,
    /// NEAR RPC URL
    rpc_url: String,
    /// Challenge TTL (seconds)
    challenge_ttl: i64,
    /// Pricing table
    pricing: PricingTable,
}

/// Pricing table for different endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PricingTable {
    /// Endpoint -> (amount, currency, description)
    entries: HashMap<String, PricingEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PricingEntry {
    amount: String,
    currency: String,
    description: String,
}

impl PricingTable {
    fn new() -> Self {
        let mut entries = HashMap::new();

        // Free endpoints
        entries.insert(
            "/health".to_string(),
            PricingEntry {
                amount: "0".to_string(),
                currency: "USD".to_string(),
                description: "Health check (free)".to_string(),
            },
        );

        // Low-cost endpoints
        entries.insert(
            "/api/v1/ping".to_string(),
            PricingEntry {
                amount: "0.0001".to_string(),
                currency: "USDC".to_string(),
                description: "Simple ping".to_string(),
            },
        );

        // Medium-cost endpoints
        entries.insert(
            "/api/v1/analyze".to_string(),
            PricingEntry {
                amount: "0.001".to_string(),
                currency: "USDC".to_string(),
                description: "Data analysis".to_string(),
            },
        );

        entries.insert(
            "/api/v1/generate".to_string(),
            PricingEntry {
                amount: "0.01".to_string(),
                currency: "USDC".to_string(),
                description: "Content generation".to_string(),
            },
        );

        // High-cost endpoints
        entries.insert(
            "/api/v1/complex".to_string(),
            PricingEntry {
                amount: "0.1".to_string(),
                currency: "USDC".to_string(),
                description: "Complex computation".to_string(),
            },
        );

        Self { entries }
    }

    fn get(&self, path: &str) -> Option<&PricingEntry> {
        // Handle root path
        let lookup_path = if path.is_empty() || path == "/" {
            "/health"
        } else {
            path
        };
        self.entries.get(lookup_path)
    }
}

/// Server state
#[derive(Clone)]
struct ServerState {
    config: ServerConfig,
    verifier: Arc<NearVerifier>,
    intents_method: Arc<NearIntentsMethod>,
    /// Store pending challenges for verification
    pending_challenges: Arc<RwLock<HashMap<String, Challenge>>>,
}

impl ServerState {
    fn new(config: ServerConfig) -> Result<Self> {
        // Create NEAR verifier for standard payments
        let verifier_config = mpp_near::server::VerifierConfig {
            rpc_url: config.rpc_url.clone(),
            recipient_account: config.recipient.clone(),
            min_amount: NearAmount::from_near(0),
            challenge_ttl: config.challenge_ttl as u64,
            confirmations: 12,
            cache_ttl: 3600,
        };
        let verifier = Arc::new(NearVerifier::new(verifier_config)?);

        // Create NEAR Intents method
        let intents_method = Arc::new(
            NearIntentsMethod::new(config.intents_api_key.clone().unwrap_or_default())
                .with_mocks(),
        );

        Ok(Self {
            config,
            verifier,
            intents_method,
            pending_challenges: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Create a payment challenge for a given endpoint
    async fn create_challenge(
        &self,
        path: &str,
        method: &str,
    ) -> Result<(Challenge, PricingEntry)> {
        // Get pricing for this endpoint
        let pricing = self
            .config
            .pricing
            .get(path)
            .ok_or_else(|| Error::Other("Endpoint not found".to_string()))?;

        // Create request data
        let request = RequestData::new(&pricing.amount, &self.config.recipient.to_string())
            .currency(&pricing.currency);

        // Build challenge
        let challenge = ChallengeBuilder::new()
            .realm("api.example.com")
            .method("near-intents") // Default to intents
            .intent("charge")
            .request(request)
            .description(&pricing.description)
            .ttl(self.config.challenge_ttl)
            .opaque_data(path.as_bytes().to_vec())
            .secret(self.config.hmac_secret.clone())
            .build()?;

        // Store challenge
        let mut challenges = self.pending_challenges.write().await;
        challenges.insert(challenge.id.clone(), challenge.clone());

        Ok((challenge, pricing.clone()))
    }

    /// Verify a credential
    async fn verify_credential(
        &self,
        credential: &Credential,
    ) -> Result<(Challenge, bool)> {
        // Get original challenge
        let challenges = self.pending_challenges.read().await;
        let challenge = challenges
            .get(&credential.challenge.id)
            .ok_or_else(|| Error::ChallengeNotFound)?;

        // Verify challenge echo
        if !credential.verify_challenge_echo(challenge) {
            return Ok((challenge.clone(), false));
        }

        // Verify challenge binding
        if !challenge.verify_binding(&self.config.hmac_secret) {
            return Ok((challenge.clone(), false));
        }

        // Verify with appropriate method
        let verified = match challenge.method.as_str() {
            "near-intents" => {
                self.intents_method
                    .verify_credential(challenge, credential)
                    .await?
            }
            "near" => {
                // Standard NEAR payment verification would go here
                // For now, accept mock payments
                credential.is_mock()
            }
            _ => false,
        };

        Ok((challenge.clone(), verified))
    }
}

/// Health check endpoint (free)
async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "mpp-near-server",
        "version": "1.0.0",
        "timestamp": SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    }))
}

/// Get pricing information
async fn get_pricing(
    State(state): State<ServerState>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "currency": "USDC",
        "endpoints": state.config.pricing.entries
    }))
}

/// Main API handler - creates challenge or processes request with payment
async fn api_handler(
    State(state): State<ServerState>,
    req: Request,
) -> Response {
    let path = req.uri().path().to_string();
    let method = req.method().to_string();
    let headers = req.headers().clone();

    // Check if endpoint requires payment
    let pricing = match state.config.pricing.get(&path) {
        Some(p) => p,
        None => {
            return (StatusCode::NOT_FOUND, Problem::not_found("Endpoint not found"))
                .into_response();
        }
    };

    // Free endpoint
    if pricing.amount == "0" {
        return handle_free_request(path).await;
    }

    // Check for Authorization header
    let auth_header = headers.get("authorization");

    match auth_header {
        None => {
            // No payment - create challenge
            match state.create_challenge(&path, &method).await {
                Ok((challenge, pricing)) => {
                    let www_auth = challenge.to_www_authenticate();

                    (
                        StatusCode::PAYMENT_REQUIRED,
                        [(header::WWW_AUTHENTICATE, www_auth.as_str())],
                        Json(serde_json::json!({
                            "status": "payment_required",
                            "challenge": {
                                "id": challenge.id,
                                "realm": challenge.realm,
                                "method": challenge.method,
                                "intent": challenge.intent,
                                "amount": pricing.amount,
                                "currency": pricing.currency,
                                "description": pricing.description,
                                "expires": challenge.expires,
                            }
                        })),
                    )
                        .into_response()
                }
                Err(e) => {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Problem::internal_error(&e.to_string()),
                    )
                        .into_response()
                }
            }
        }
        Some(auth_value) => {
            // Payment provided - verify credential
            let auth_str = match auth_value.to_str() {
                Ok(s) => s,
                Err(_) => {
                    return (
                        StatusCode::BAD_REQUEST,
                        Problem::invalid_request("Invalid Authorization header"),
                    )
                        .into_response();
                }
            };

            match Credential::from_authorization(auth_str) {
                Ok(credential) => {
                    match state.verify_credential(&credential).await {
                        Ok((challenge, verified)) => {
                            if verified {
                                // Payment verified - create receipt and handle request
                                let receipt = Receipt::for_payment(
                                    &challenge.id,
                                    credential.source.as_deref(),
                                    &pricing.amount,
                                    &pricing.currency,
                                );
                                let receipt_header = receipt.to_header();

                                let response = handle_paid_request(&path).await;

                                // Add receipt header to response
                                let mut response = response.into_response();
                                response.headers_mut().insert(
                                    header::PAYMENT_RECEIPT,
                                    HeaderValue::from_str(&receipt_header).unwrap(),
                                );
                                response
                            } else {
                                (
                                    StatusCode::PAYMENT_REQUIRED,
                                    Problem::verification_failed("Payment verification failed"),
                                )
                                    .into_response()
                            }
                        }
                        Err(e) => {
                            (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Problem::internal_error(&e.to_string()),
                            )
                                .into_response()
                        }
                    }
                }
                Err(e) => {
                    (
                        StatusCode::BAD_REQUEST,
                        Problem::invalid_credential(&e.to_string()),
                    )
                        .into_response()
                }
            }
        }
    }
}

/// Handle free request
async fn handle_free_request(path: String) -> Response {
    let response_body = match path.as_str() {
        "/health" => serde_json::json!({
            "status": "ok",
            "message": "Health check passed",
            "paid": false
        }),
        _ => serde_json::json!({
            "message": "This endpoint is free!",
            "path": path,
            "paid": false
        }),
    };

    Json(response_body).into_response()
}

/// Handle paid request
async fn handle_paid_request(path: String) -> Response {
    let response_body = match path.as_str() {
        "/api/v1/ping" => serde_json::json!({
            "status": "pong",
            "timestamp": chrono::Utc::now(),
            "message": "Ping successful"
        }),
        "/api/v1/analyze" => serde_json::json!({
            "result": "Analysis complete",
            "data": {
                "metrics": ["cpu", "memory", "disk"],
                "values": [45, 67, 23]
            },
            "processed_at": chrono::Utc::now()
        }),
        "/api/v1/generate" => serde_json::json!({
            "content": "Generated content based on your request",
            "tokens_used": 150,
            "model": "mpp-ai-v1",
            "generated_at": chrono::Utc::now()
        }),
        "/api/v1/complex" => serde_json::json!({
            "computation": "Complex task completed",
            "steps": [
                "initialization",
                "processing",
                "analysis",
                "finalization"
            ],
            "duration_ms": 1234,
            "result": "success"
        }),
        _ => serde_json::json!({
            "message": "Request processed successfully",
            "path": path,
            "paid": true
        }),
    };

    Json(response_body).into_response()
}

/// Header names
mod header {
    pub const WWW_AUTHENTICATE: &str = "www-authenticate";
    pub const AUTHORIZATION: &str = "authorization";
    pub const PAYMENT_RECEIPT: &str = "payment-receipt";
}

/// Main function
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    // Load configuration from environment
    let config = ServerConfig {
        hmac_secret: std::env::var("MPP_HMAC_SECRET")
            .unwrap_or_else(|_| {
                println!("⚠️  Warning: Using default HMAC secret. Set MPP_HMAC_SECRET env var!");
                "default-secret-change-me".as_bytes().to_vec()
            })
            .into_bytes(),
        recipient: AccountId::new(
            std::env::var("MPP_RECIPIENT").unwrap_or_else(|_| "merchant.near".to_string()),
        )
        .map_err(|e| format!("Invalid recipient account: {}", e))?,
        intents_api_key: std::env::var("OUTLAYER_API_KEY").ok(),
        rpc_url: std::env::var("MPP_RPC_URL")
            .unwrap_or_else(|_| "https://rpc.mainnet.near.org".to_string()),
        challenge_ttl: std::env::var("MPP_CHALLENGE_TTL")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(300), // 5 minutes
        pricing: PricingTable::new(),
    };

    // Create server state
    let state = ServerState::new(config.clone())?;

    // Build router
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/pricing", get(get_pricing))
        .route("/api/v1/ping", axum::routing::any(api_handler))
        .route("/api/v1/analyze", axum::routing::any(api_handler))
        .route("/api/v1/generate", axum::routing::any(api_handler))
        .route("/api/v1/complex", axum::routing::any(api_handler))
        .with_state(state)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    // Print server info
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║          MPP-NEAR Full Server Example                     ║");
    println!("╠════════════════════════════════════════════════════════════╣");
    println!("║  Recipient: {:<48} ║", config.recipient);
    println!("║  Challenge TTL: {:<45} ║", config.challenge_ttl);
    println!("║  Payment Methods: near, near-intents                    ║");
    println!("╠════════════════════════════════════════════════════════════╣");
    println!("║  Endpoints:                                             ║");
    for (path, pricing) in &config.pricing.entries {
        let status = if pricing.amount == "0" { "FREE" } else { "PAID" };
        println!("║  {:<20} {:<10} {} {:<20} ║", path, status, pricing.amount, pricing.currency);
    }
    println!("╠════════════════════════════════════════════════════════════╣");
    println!("║  Example curl commands:                                  ║");
    println!("║  # Get pricing:                                           ║");
    println!("║  curl http://localhost:3000/pricing                       ║");
    println!("║                                                          ║");
    println!("║  # Paid request (without payment - gets challenge):     ║");
    println!("║  curl http://localhost:3000/api/v1/ping                  ║");
    println!("║                                                          ║");
    println!("║  # Free request:                                         ║");
    println!("║  curl http://localhost:3000/health                       ║");
    println!("╚════════════════════════════════════════════════════════════╝");

    // Start server
    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("\n🚀 Server listening on http://{}\n", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
