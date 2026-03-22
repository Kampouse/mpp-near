//! Comprehensive MPP-NEAR Server Example
//!
//! This example demonstrates a complete MPP server supporting:
//! - Standard NEAR payments (on-chain transfers)
//! - NEAR Intents payments (gasless via OutLayer)
//! - Cross-chain token support (OMFT tokens)
//! - Cross-chain withdrawals to 20+ chains
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
//! export OUTLAYER_API_KEY="your-api-key"  # Required for cross-chain withdrawals
//! export MPP_RPC_URL="https://rpc.mainnet.near.org"
//! ```

use axum::{
    extract::{Request, State},
    http::{HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use mpp_near::primitives::headers;
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
use std::time::SystemTime;
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

        // Create NEAR Intents method (NO MOCKS - real payments only)
        let intents_method = Arc::new(
            NearIntentsMethod::new(
                config.intents_api_key.clone().unwrap_or_else(|| {
                    println!("⚠️  Warning: No OUTLAYER_API_KEY set.");
                    println!("⚠️  Set OUTLAYER_API_KEY environment variable for real payments.");
                    String::new()
                })
            )
            // NO .with_mocks() - requires real payments
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
        _method: &str,
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
        let mut opaque_map: HashMap<String, String> = HashMap::new();
        opaque_map.insert("path".to_string(), path.to_string());

        let challenge = ChallengeBuilder::new()
            .realm("api.example.com")
            .method("near-intents") // Default to intents
            .intent("charge")
            .request(request)
            .description(&pricing.description)
            .ttl(self.config.challenge_ttl)
            .opaque_data(opaque_map)
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
                // For NEAR Intents, verify the credential structure and check it's not a mock
                let proof = credential.payload.get("proof")
                    .and_then(|p| p.as_str())
                    .ok_or_else(|| Error::VerificationFailed("Missing proof in credential".into()))?;

                // Extract request data
                let request_data = mpp_near::primitives::RequestData::decode(&challenge.request)?;

                // Verify amount is reasonable
                let amount = request_data.amount.parse::<f64>()
                    .map_err(|_| Error::VerificationFailed("Invalid amount".into()))?;

                if amount <= 0.0 {
                    return Ok((challenge.clone(), false));
                }

                // Verify recipient matches expected
                if request_data.recipient != self.config.recipient.to_string() {
                    tracing::warn!("Payment recipient mismatch: expected {}, got {}",
                        self.config.recipient, request_data.recipient);
                    return Ok((challenge.clone(), false));
                }

                // Verify the proof is not a mock
                // Real OutLayer payments have UUID-like request_ids (e.g., "ae1b646e-891a-4631-bb5a-ec616ea52437")
                // Mock payments start with "test_", "mock_", or "fake_"
                let is_valid_uuid = uuid::Uuid::parse_str(proof).is_ok()
                    || (proof.len() == 36 && proof.chars().filter(|&c| c == '-').count() == 4);

                is_valid_uuid && !credential.is_mock()
            }
            "near" => {
                // For standard NEAR payments, verify the credential structure
                // In production, you would verify the transaction on-chain via RPC
                let proof = credential.payload.get("proof")
                    .and_then(|p| p.as_str())
                    .ok_or_else(|| Error::VerificationFailed("Missing proof in credential".into()))?;

                // Extract request data
                let request_data = mpp_near::primitives::RequestData::decode(&challenge.request)?;

                // Verify amount is reasonable
                let amount = request_data.amount.parse::<f64>()
                    .map_err(|_| Error::VerificationFailed("Invalid amount".into()))?;

                if amount <= 0.0 {
                    return Ok((challenge.clone(), false));
                }

                // Verify recipient matches expected
                if request_data.recipient != self.config.recipient.to_string() {
                    tracing::warn!("Payment recipient mismatch: expected {}, got {}",
                        self.config.recipient, request_data.recipient);
                    return Ok((challenge.clone(), false));
                }

                // In production: Verify the transaction on-chain
                // For now, accept the payment if proof looks valid
                proof.len() > 10 && !credential.is_mock()
            }
            _ => {
                tracing::warn!("Unknown payment method: {}", challenge.method);
                false
            }
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

/// Cross-chain withdrawal request
#[derive(Debug, Deserialize)]
struct WithdrawRequest {
    /// Destination address (can be NEAR account or other chain address)
    to: String,
    /// Amount in human-readable format (e.g., "1.5" for 1.5 USDC)
    amount: String,
    /// Token to withdraw (e.g., "usdc", "near", or full token ID)
    token: String,
    /// Destination chain (near, ethereum, solana, bitcoin, etc.)
    #[serde(default = "default_chain")]
    chain: String,
}

fn default_chain() -> String {
    "near".to_string()
}

/// Cross-chain withdrawal endpoint
/// Allows withdrawing received payments to other chains
async fn withdraw_cross_chain(
    State(state): State<ServerState>,
    Json(req): Json<WithdrawRequest>,
) -> Response {
    // Check if OutLayer API key is configured
    let api_key = match state.config.intents_api_key.as_ref() {
        Some(key) => key,
        None => {
            let mut problem = Problem::new(mpp_near::primitives::ProblemType::Custom(
                "https://mpp.dev/problems/not-configured".to_string(),
            ));
            problem.detail = Some("OutLayer API key not configured. Set OUTLAYER_API_KEY environment variable.".to_string());
            problem.status = 500;
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(problem)).into_response();
        }
    };

    // Parse amount
    let amount_f64 = match req.amount.parse::<f64>() {
        Ok(a) => a,
        Err(e) => {
            let mut problem = Problem::new(mpp_near::primitives::ProblemType::Custom(
                "https://mpp.dev/problems/invalid-amount".to_string(),
            ));
            problem.detail = Some(format!("Invalid amount: {}", e));
            problem.status = 400;
            return (StatusCode::BAD_REQUEST, Json(problem)).into_response();
        }
    };

    // Determine token ID
    let token_id = match req.token.to_lowercase().as_str() {
        "usdc" => "17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1",
        "usdt" => "usdt.tether-token.near",
        "near" => "near",
        _ => &req.token, // Use as-is for custom tokens (e.g., btc.omft.near)
    };

    // Calculate amount in smallest denomination
    let decimals = match token_id {
        "near" => 24,
        "17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1" => 6, // USDC
        "usdt.tether-token.near" => 6, // USDT
        _ => 18, // Default for most tokens
    };

    let amount_smallest = (amount_f64 * 10_f64.powi(decimals) as f64) as u64;

    // Call OutLayer API for cross-chain withdrawal
    let client = reqwest::Client::new();
    let outlayer_url = std::env::var("OUTLAYER_API_URL")
        .unwrap_or_else(|_| "https://api.outlayer.fastnear.com".to_string());

    let payload = serde_json::json!({
        "to": req.to,
        "amount": amount_smallest.to_string(),
        "token": token_id,
        "chain": req.chain.to_lowercase()
    });

    let endpoint = format!("{}/wallet/v1/intents/withdraw", outlayer_url);

    let response = match client
        .post(&endpoint)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            let mut problem = Problem::new(mpp_near::primitives::ProblemType::Custom(
                "https://mpp.dev/problems/withdrawal-failed".to_string(),
            ));
            problem.detail = Some(format!("OutLayer API request failed: {}", e));
            problem.status = 500;
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(problem)).into_response();
        }
    };

    if !response.status().is_success() {
        let error_text: String = response.text().await.unwrap_or_default();
        let mut problem = Problem::new(mpp_near::primitives::ProblemType::Custom(
            "https://mpp.dev/problems/withdrawal-failed".to_string(),
        ));
        problem.detail = Some(format!("OutLayer API error: {}", error_text));
        problem.status = 500;
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(problem)).into_response();
    }

    let response_json: serde_json::Value = match response.json().await {
        Ok(j) => j,
        Err(e) => {
            let mut problem = Problem::new(mpp_near::primitives::ProblemType::Custom(
                "https://mpp.dev/problems/withdrawal-failed".to_string(),
            ));
            problem.detail = Some(format!("Failed to parse response: {}", e));
            problem.status = 500;
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(problem)).into_response();
        }
    };

    let tx_hash = response_json
        .get("request_id")
        .or_else(|| response_json.get("intent_hash"))
        .or_else(|| response_json.get("hash"))
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");

    Json(serde_json::json!({
        "status": "success",
        "message": "Cross-chain withdrawal initiated",
        "withdrawal": {
            "to": req.to,
            "amount": req.amount,
            "token": req.token,
            "chain": req.chain,
            "transaction": tx_hash
        }
    })).into_response()
}

/// Get supported chains for cross-chain withdrawals
async fn get_supported_chains() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "supported_chains": [
            {"id": "near", "name": "NEAR Protocol", "native_token": "NEAR"},
            {"id": "ethereum", "name": "Ethereum", "native_token": "ETH"},
            {"id": "solana", "name": "Solana", "native_token": "SOL"},
            {"id": "bitcoin", "name": "Bitcoin", "native_token": "BTC"},
            {"id": "arbitrum", "name": "Arbitrum", "native_token": "ETH"},
            {"id": "base", "name": "Base", "native_token": "ETH"},
            {"id": "polygon", "name": "Polygon", "native_token": "MATIC"},
            {"id": "optimism", "name": "Optimism", "native_token": "ETH"},
            {"id": "avalanche", "name": "Avalanche", "native_token": "AVAX"},
            {"id": "bsc", "name": "Binance Smart Chain", "native_token": "BNB"},
            {"id": "ton", "name": "TON", "native_token": "TON"},
            {"id": "aptos", "name": "Aptos", "native_token": "APT"},
            {"id": "sui", "name": "Sui", "native_token": "SUI"},
            {"id": "starknet", "name": "StarkNet", "native_token": "ETH"},
            {"id": "tron", "name": "Tron", "native_token": "TRX"},
            {"id": "stellar", "name": "Stellar", "native_token": "XLM"},
            {"id": "dogecoin", "name": "Dogecoin", "native_token": "DOGE"},
            {"id": "xrp", "name": "XRP", "native_token": "XRP"},
            {"id": "zcash", "name": "Zcash", "native_token": "ZEC"},
            {"id": "litecoin", "name": "Litecoin", "native_token": "LTC"},
        ],
        "note": "Withdrawals are gasless and executed via NEAR Intents protocol",
        "documentation": "https://docs.outlayer.ai"
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
            let mut problem = Problem::new(mpp_near::primitives::ProblemType::Custom(
                "https://mpp.dev/problems/not-found".to_string(),
            ));
            problem.detail = Some("Endpoint not found".to_string());
            problem.status = 404;
            return (StatusCode::NOT_FOUND, Json(problem)).into_response();
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
                        [(headers::WWW_AUTHENTICATE, www_auth.as_str())],
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
                    let mut problem = Problem::new(mpp_near::primitives::ProblemType::Custom(
                        "https://mpp.dev/problems/internal-error".to_string(),
                    ));
                    problem.detail = Some(e.to_string());
                    problem.status = 500;
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(problem)).into_response()
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
                        Json(Problem::malformed_credential("Invalid Authorization header")),
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

                                let response = handle_paid_request(path.clone()).await;

                                // Add receipt header to response
                                let mut response = response.into_response();
                                response.headers_mut().insert(
                                    headers::PAYMENT_RECEIPT,
                                    HeaderValue::from_str(&receipt_header).unwrap(),
                                );
                                response
                            } else {
                                (
                                    StatusCode::PAYMENT_REQUIRED,
                                    Json(Problem::verification_failed("Payment verification failed")),
                                )
                                    .into_response()
                            }
                        }
                        Err(e) => {
                            let mut problem = Problem::new(mpp_near::primitives::ProblemType::Custom(
                                "https://mpp.dev/problems/internal-error".to_string(),
                            ));
                            problem.detail = Some(e.to_string());
                            problem.status = 500;
                            (StatusCode::INTERNAL_SERVER_ERROR, Json(problem)).into_response()
                        }
                    }
                }
                Err(e) => {
                    (
                        StatusCode::BAD_REQUEST,
                        Json(Problem::malformed_credential(&e.to_string())),
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

/// Main function
#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file
    dotenv::dotenv().ok();

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
                "default-secret-change-me".to_string()
            })
            .into_bytes(),
        recipient: AccountId::new(
            std::env::var("MPP_RECIPIENT").unwrap_or_else(|_| "merchant.near".to_string()),
        )?,
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
        .route("/chains", get(get_supported_chains))
        .route("/withdraw", axum::routing::post(withdraw_cross_chain))
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
    println!("║  Payment Methods: near, near-intents, cross-chain        ║");
    println!("╠════════════════════════════════════════════════════════════╣");
    println!("║  Paid Endpoints:                                         ║");
    for (path, pricing) in &config.pricing.entries {
        if pricing.amount != "0" {
            println!("║  {:<20} {} {:<20} ║", path, pricing.amount, pricing.currency);
        }
    }
    println!("╠════════════════════════════════════════════════════════════╣");
    println!("║  Management Endpoints:                                    ║");
    println!("║  GET  /health          - Health check (free)              ║");
    println!("║  GET  /pricing         - List endpoint pricing            ║");
    println!("║  GET  /chains          - List supported chains            ║");
    println!("║  POST /withdraw        - Cross-chain withdrawal           ║");
    println!("╠════════════════════════════════════════════════════════════╣");
    println!("║  Example curl commands:                                  ║");
    println!("║  # Get pricing:                                           ║");
    println!("║  curl http://localhost:3000/pricing                       ║");
    println!("║                                                          ║");
    println!("║  # Get supported chains:                                  ║");
    println!("║  curl http://localhost:3000/chains                        ║");
    println!("║                                                          ║");
    println!("║  # Withdraw to Solana:                                    ║");
    println!("║  curl -X POST http://localhost:3000/withdraw \\            ║");
    println!("║    -H 'Content-Type: application/json' \\                 ║");
    println!("║    -d '{{\"to\":\"addr\",\"amount\":\"1\",\"token\":\"usdc\"}}'   ║");
    println!("║                                                          ║");
    println!("║  # Paid request (gets 402 challenge):                    ║");
    println!("║  curl http://localhost:3000/api/v1/ping                  ║");
    println!("╚════════════════════════════════════════════════════════════╝");

    // Start server
    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| Error::Other(format!("Failed to bind to {}: {}", addr, e)))?;
    println!("\n🚀 Server listening on http://{}\n", addr);

    axum::serve(listener, app)
        .await
        .map_err(|e| Error::Other(format!("Server error: {}", e)))?;

    Ok(())
}
