//! Example: Server accepting NEAR payments

#[cfg(feature = "server")]
use axum::{
    routing::get,
    Router,
    Json,
};
use mpp_near::server::{NearVerifier, VerifierConfig};
use mpp_near::types::{AccountId, NearAmount};
use serde_json::json;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup logging
    tracing_subscriber::fmt::init();
    
    // Configure verifier
    let config = VerifierConfig {
        rpc_url: "https://rpc.mainnet.near.org".to_string(),
        recipient_account: AccountId::new("merchant.near")?,
        min_amount: NearAmount::from_near(1), // Minimum 1 NEAR
        challenge_ttl: 300, // 5 minutes
        confirmations: 12,
        ..Default::default()
    };
    
    println!("Setting up NEAR payment verifier for: {}", config.recipient_account);
    
    let verifier = NearVerifier::new(config)?;
    let verifier = Arc::new(verifier);
    
    // Build router (simplified - no payment extraction for now)
    let app = Router::new()
        .route("/free", get(free_endpoint))
        .route("/challenge", get(create_challenge))
        .with_state(verifier);
    
    // Start server
    let addr = "0.0.0.0:3000";
    println!("\nServer listening on {}", addr);
    println!("Endpoints:");
    println!("  GET /free - Free endpoint");
    println!("  GET /challenge - Create payment challenge");
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

/// Free endpoint - no payment required
async fn free_endpoint() -> Json<serde_json::Value> {
    Json(json!({
        "message": "This endpoint is free!",
        "paid": false
    }))
}

/// Create a payment challenge
async fn create_challenge(
    axum::extract::State(verifier): axum::extract::State<Arc<NearVerifier>>,
) -> Json<serde_json::Value> {
    match verifier.charge("1").await {
        Ok(challenge) => Json(json!({
            "status": "payment_required",
            "challenge": challenge
        })),
        Err(e) => Json(json!({
            "error": e.to_string()
        })),
    }
}
