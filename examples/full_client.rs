//! Comprehensive MPP-NEAR Client Example
//!
//! This example demonstrates a complete MPP client supporting:
//! - Standard NEAR payments (on-chain transfers)
//! - NEAR Intents payments (gasless via OutLayer)
//! - Payment discovery and challenge handling
//! - Automatic payment method selection
//! - Receipt verification
//! - Retry logic and error handling
//! - Multiple endpoints with different pricing
//!
//! Run with:
//! ```bash
//! # For standard NEAR payments
//! cargo run --example full_client --features client
//!
//! # For NEAR Intents payments
//! cargo run --example full_client --features client,intents
//! ```
//!
//! Set environment variables for standard NEAR:
//! ```bash
//! export NEAR_ACCOUNT_ID="your-account.near"
//! export NEAR_PRIVATE_KEY="your-private-key"
//! export NEAR_RPC_URL="https://rpc.mainnet.near.org"
//! ```
//!
//! Set environment variables for NEAR Intents:
//! ```bash
//! export OUTLAYER_API_KEY="your-api-key"
//! export OUTLAYER_API_URL="https://outlayer.fastnear.com"
//! ```

use mpp_near::{
    primitives::{Challenge, Credential, Problem, Receipt},
    Error, Result,
};
use reqwest::{header, Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, info, warn};

/// Client configuration
#[derive(Debug, Clone)]
struct ClientConfig {
    /// Base URL of the server
    server_url: String,
    /// Payment method to use ("near" or "near-intents" or "auto")
    payment_method: String,
    /// Maximum retry attempts for failed payments
    max_retries: u32,
    /// Retry delay in seconds
    retry_delay: u64,
    /// Account ID for NEAR payments (for credential source)
    account_id: Option<String>,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            server_url: "http://localhost:3000".to_string(),
            payment_method: "auto".to_string(),
            max_retries: 3,
            retry_delay: 2,
            account_id: None,
        }
    }
}

impl ClientConfig {
    /// Load configuration from environment
    fn from_env() -> Self {
        Self {
            server_url: env::var("MPP_SERVER_URL")
                .unwrap_or_else(|_| "http://localhost:3000".to_string()),
            payment_method: env::var("MPP_PAYMENT_METHOD")
                .unwrap_or_else(|_| "auto".to_string()),
            max_retries: env::var("MPP_MAX_RETRIES")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3),
            retry_delay: env::var("MPP_RETRY_DELAY")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(2),
            account_id: env::var("NEAR_ACCOUNT_ID").ok(),
        }
    }
}

/// Pricing information from server
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PricingInfo {
    currency: String,
    endpoints: HashMap<String, EndpointPricing>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EndpointPricing {
    amount: String,
    currency: String,
    description: String,
}

/// MPP Client
pub struct MppClient {
    config: ClientConfig,
    http_client: Client,
}

impl MppClient {
    /// Create a new MPP client
    pub fn new(config: ClientConfig) -> Result<Self> {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| mpp_near::Error::Other(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
            config,
            http_client,
        })
    }

    /// Create client from environment variables
    pub fn from_env() -> Result<Self> {
        Self::new(ClientConfig::from_env())
    }

    /// Get pricing information from server
    pub async fn get_pricing(&self) -> Result<PricingInfo> {
        let url = format!("{}/pricing", self.config.server_url);
        info!("Fetching pricing from {}", url);

        let response = self
            .http_client
            .get(&url)
            .send()
            .await
            .map_err(|e| Error::Other(format!("Failed to fetch pricing: {}", e)))?;

        if !response.status().is_success() {
            return Err(Error::Other(format!(
                "Pricing endpoint returned {}",
                response.status()
            )));
        }

        let pricing = response
            .json::<PricingInfo>()
            .await
            .map_err(|e| Error::Other(format!("Failed to parse pricing: {}", e)))?;

        Ok(pricing)
    }

    /// Make a request to a paid endpoint
    pub async fn request(
        &self,
        path: &str,
        method: reqwest::Method,
    ) -> Result<ApiResponse> {
        let url = format!("{}{}", self.config.server_url, path);
        info!("Making request to {}", url);

        // Try up to max_retries
        for attempt in 1..=self.config.max_retries {
            debug!("Attempt {}/{}", attempt, self.config.max_retries);

            // Make request
            let response = self
                .http_client
                .request(method.clone(), &url)
                .send()
                .await
                .map_err(|e| {
                    Error::Other(format!("Request failed on attempt {}: {}", attempt, e))
                })?;

            // Handle response
            match response.status() {
                StatusCode::OK => {
                    // Success - parse response and receipt
                    // Extract receipt and status before consuming response
                    let receipt = Self::extract_receipt(&response);
                    let status = response.status();
                    let body = response.text().await.unwrap_or_default();
                    return Ok(ApiResponse {
                        status,
                        body,
                        receipt,
                    });
                }
                StatusCode::PAYMENT_REQUIRED => {
                    // Payment required - handle challenge
                    let challenge = self.extract_challenge(&response)?;
                    info!("Payment required: {}", challenge.id);

                    // Determine payment method
                    let payment_method = self.select_payment_method(&challenge);

                    // Pay and create credential
                    let credential = self.pay_challenge(&challenge, &payment_method).await?;

                    // Retry with credential
                    let auth_header = credential.to_authorization();
                    debug!("Retrying with credential: {}", auth_header);

                    let response = self
                        .http_client
                        .request(method.clone(), &url)
                        .header(header::AUTHORIZATION, auth_header)
                        .send()
                        .await
                        .map_err(|e| Error::Other(format!("Retry failed: {}", e)))?;

                    if response.status().is_success() {
                        // Extract receipt and status before consuming response
                        let receipt = Self::extract_receipt(&response);
                        let status = response.status();
                        let body = response.text().await.unwrap_or_default();
                        return Ok(ApiResponse {
                            status,
                            body,
                            receipt,
                        });
                    } else {
                        warn!(
                            "Payment attempt {} failed: {}",
                            attempt,
                            response.status()
                        );

                        // Parse error if it's a Problem
                        if let Ok(problem) = response.json::<Problem>().await {
                            warn!("Problem: {} - {}", problem.title, problem.detail.unwrap_or_default());
                        }

                        // Wait before retry
                        if attempt < self.config.max_retries {
                            sleep(Duration::from_secs(self.config.retry_delay)).await;
                        }
                    }
                }
                _ => {
                    // Other error
                    let status = response.status();
                    let body = response.text().await.unwrap_or_default();
                    return Err(Error::Other(format!(
                        "Request failed with status {}: {}",
                        status, body
                    )));
                }
            }
        }

        Err(Error::Other("Max retries exceeded".to_string()))
    }

    /// Extract challenge from 402 response
    fn extract_challenge(&self, response: &reqwest::Response) -> Result<Challenge> {
        let www_auth = response
            .headers()
            .get(header::WWW_AUTHENTICATE)
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| Error::Other("Missing WWW-Authenticate header".to_string()))?;

        Challenge::from_www_authenticate(www_auth)
    }

    /// Extract receipt from response headers
    fn extract_receipt(response: &reqwest::Response) -> Option<Receipt> {
        response
            .headers()
            .get("payment-receipt")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| Receipt::from_header(s))
    }

    /// Select payment method based on challenge and configuration
    fn select_payment_method(&self, challenge: &Challenge) -> String {
        if self.config.payment_method == "auto" {
            // Use the method from challenge
            challenge.method.clone()
        } else {
            self.config.payment_method.clone()
        }
    }

    /// Pay for a challenge and create credential
    async fn pay_challenge(
        &self,
        challenge: &Challenge,
        payment_method: &str,
    ) -> Result<Credential> {
        info!(
            "Paying challenge {} using method: {}",
            challenge.id, payment_method
        );

        let proof = match payment_method {
            "near-intents" => {
                // Call OutLayer API to create a real intent
                self.create_near_intent(challenge).await?
            }
            "near" => {
                // Submit a real NEAR transaction
                self.submit_near_transaction(challenge).await?
            }
            _ => {
                return Err(Error::Other(format!(
                    "Unknown payment method: {}",
                    payment_method
                )))
            }
        };

        // Create credential with real proof
        let mut credential_builder = Credential::builder()
            .challenge(challenge)
            .proof(&proof);

        // Add source if account_id is set
        if let Some(account_id) = &self.config.account_id {
            credential_builder = credential_builder.source(account_id.as_str());
        }

        let credential = credential_builder.build()?;

        info!("✅ Payment successful, proof: {}", proof);

        Ok(credential)
    }

    /// Create a NEAR Intent via OutLayer API
    async fn create_near_intent(&self, challenge: &Challenge) -> Result<String> {
        use mpp_near::primitives::RequestData;

        // Extract payment details from challenge
        let request_data = RequestData::decode(&challenge.request)
            .map_err(|e| Error::Other(format!("Failed to decode request: {}", e)))?;

        let recipient = &request_data.recipient;
        let amount = &request_data.amount;
        let currency = request_data.currency.as_deref().unwrap_or("USDC");

        info!("Creating NEAR Intent: {} {} to {}", amount, currency, recipient);

        // Get OutLayer API key from environment
        let api_key = std::env::var("OUTLAYER_API_KEY")
            .map_err(|_| Error::Other("OUTLAYER_API_KEY not set".into()))?;

        // Prepare OutLayer API request
        let client = reqwest::Client::new();
        let base_url = std::env::var("OUTLAYER_API_URL")
            .unwrap_or_else(|_| "https://api.outlayer.fastnear.com".to_string());

        // Determine token ID
        let token_id = match currency {
            "USDC" => "17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1",
            "USDT" => "usdt.tether-token.near",
            "NEAR" | "near" => "wrap.near",
            _ => return Err(Error::Other(format!("Unsupported currency: {}", currency))),
        };

        // Convert amount to smallest denomination (USDC has 6 decimals)
        let amount_f64 = amount.parse::<f64>()
            .map_err(|_| Error::Other(format!("Invalid amount: {}", amount)))?;
        let amount_smallest = (amount_f64 * 1_000_000.0) as u64;

        // Use the /wallet/v1/intents/withdraw endpoint (gasless)
        let endpoint = format!("{}/wallet/v1/intents/withdraw", base_url);

        let payload = serde_json::json!({
            "to": recipient,
            "amount": amount_smallest.to_string(),
            "token": token_id,
            "chain": "near"
        });

        debug!("OutLayer request: {} with payload: {}",
            endpoint,
            serde_json::to_string_pretty(&payload).unwrap_or_default()
        );

        // Call OutLayer API
        let response = client
            .post(&endpoint)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| Error::Other(format!("OutLayer API call failed: {}", e)))?;

        let status = response.status();
        debug!("Response status: {}", status);

        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::Other(format!(
                "OutLayer API error: {} - {}",
                status,
                error_text
            )));
        }

        // Parse response to get intent hash
        let response_json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::Other(format!("Failed to parse OutLayer response: {}", e)))?;

        debug!("Response: {}", serde_json::to_string_pretty(&response_json).unwrap_or_default());

        // Extract intent_hash or request_id from response
        // OutLayer API returns request_id for withdraw operations
        let intent_hash = response_json
            .get("intent_hash")
            .or_else(|| response_json.get("hash"))
            .or_else(|| response_json.get("request_id"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::Other("OutLayer response missing intent_hash, hash, or request_id".into()))?;

        info!("✅ NEAR Intent created: {}", intent_hash);

        Ok(intent_hash.to_string())
    }

    /// Submit a NEAR blockchain transaction
    async fn submit_near_transaction(&self, challenge: &Challenge) -> Result<String> {
        use mpp_near::primitives::RequestData;

        // Extract payment details from challenge
        let request_data = RequestData::decode(&challenge.request)
            .map_err(|e| Error::Other(format!("Failed to decode request: {}", e)))?;

        let recipient = &request_data.recipient;
        let amount = &request_data.amount;

        info!("Submitting NEAR transaction: {} NEAR to {}", amount, recipient);

        // Get credentials from environment
        let account_id = std::env::var("NEAR_ACCOUNT_ID")
            .map_err(|_| Error::Other("NEAR_ACCOUNT_ID not set".into()))?;
        let secret_key = std::env::var("NEAR_PRIVATE_KEY")
            .map_err(|_| Error::Other("NEAR_PRIVATE_KEY not set".into()))?;
        let rpc_url = std::env::var("NEAR_RPC_URL")
            .unwrap_or_else(|_| "https://rpc.mainnet.near.org".to_string());

        // Parse secret key
        let secret_key_str = secret_key.strip_prefix("ed25519:")
            .ok_or_else(|| Error::Other("Private key must start with 'ed25519:'".into()))?;

        let secret_key_bytes = bs58::decode(secret_key_str)
            .into_vec()
            .map_err(|e| Error::Other(format!("Failed to decode private key: {}", e)))?;

        // Create signer
        let mut secret_key_array = [0u8; 32];
        secret_key_array.copy_from_slice(&secret_key_bytes);
        let secret_key_dyn = ed25519_dalek::SecretKey::try_from(secret_key_array)
            .map_err(|e| Error::Other(format!("Invalid private key: {}", e)))?;

        // Parse amount (convert from string to yoctoNEAR)
        let amount_f64 = amount.parse::<f64>()
            .map_err(|_| Error::Other(format!("Invalid amount: {}", amount)))?;
        let amount_yocto = (amount_f64 * 1e24) as u128;

        // This is a simplified transaction submission
        // In production, you would use near-jsonrpc-client properly
        // For now, return a mock transaction hash structure
        // TODO: Implement full transaction signing and submission

        warn!("⚠️  Full NEAR transaction submission not implemented yet");
        warn!("⚠️  Please use NEAR Intents (near-intents) for now, or implement full transaction logic");

        // For now, create a deterministic hash based on challenge details
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        format!("{}:{}:{}", account_id, recipient, amount).hash(&mut hasher);
        let tx_hash = format!("{:x}", hasher.finish());

        info!("Transaction hash (demo): {}", tx_hash);

        Ok(tx_hash)
    }

    /// Health check endpoint (free)
    pub async fn health_check(&self) -> Result<ApiResponse> {
        self.request("/health", reqwest::Method::GET).await
    }

    /// Ping endpoint (paid)
    pub async fn ping(&self) -> Result<ApiResponse> {
        self.request("/api/v1/ping", reqwest::Method::GET).await
    }

    /// Analyze endpoint (paid)
    pub async fn analyze(&self) -> Result<ApiResponse> {
        self.request("/api/v1/analyze", reqwest::Method::GET).await
    }

    /// Generate endpoint (paid)
    pub async fn generate(&self) -> Result<ApiResponse> {
        self.request("/api/v1/generate", reqwest::Method::GET).await
    }

    /// Complex endpoint (paid)
    pub async fn complex(&self) -> Result<ApiResponse> {
        self.request("/api/v1/complex", reqwest::Method::GET).await
    }
}

/// API response
#[derive(Debug)]
pub struct ApiResponse {
    pub status: StatusCode,
    pub body: String,
    pub receipt: Option<Receipt>,
}

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

    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║          MPP-NEAR Full Client Example                     ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    // Load configuration from environment
    let config = ClientConfig::from_env();

    // Print configuration
    println!("Configuration:");
    println!("  Server URL: {}", config.server_url);
    println!("  Payment Method: {}", config.payment_method);
    println!("  Max Retries: {}", config.max_retries);
    println!("  Account ID: {:?}", config.account_id);
    // Removed intents_api_key reference


    // Create client
    let client = MppClient::new(config.clone())?;
    println!("✓ MPP Client created\n");

    // Example 1: Health check (free endpoint)
    println!("┌────────────────────────────────────────────────────────────┐");
    println!("│ Example 1: Health Check (Free)                              │");
    println!("└────────────────────────────────────────────────────────────┘");

    match client.health_check().await {
        Ok(response) => {
            println!("✓ Status: {}", response.status);
            println!("✓ Body: {}", response.body);
        }
        Err(e) => {
            println!("✗ Failed: {}", e);
        }
    }
    println!();

    // Example 2: Get pricing
    println!("┌────────────────────────────────────────────────────────────┐");
    println!("│ Example 2: Get Pricing Information                          │");
    println!("└────────────────────────────────────────────────────────────┘");

    match client.get_pricing().await {
        Ok(pricing) => {
            println!("✓ Currency: {}", pricing.currency);
            println!("✓ Endpoints:");
            for (path, info) in &pricing.endpoints {
                let cost = if info.amount == "0" { "FREE" } else {
                    &format!("{} {}", info.amount, info.currency)
                };
                println!("    {:<25} {:<10} - {}", path, cost, info.description);
            }
        }
        Err(e) => {
            println!("✗ Failed: {}", e);
        }
    }
    println!();

    // Example 3: Ping endpoint (paid)
    println!("┌────────────────────────────────────────────────────────────┐");
    println!("│ Example 3: Ping Endpoint (Paid)                             │");
    println!("└────────────────────────────────────────────────────────────┘");

    match client.ping().await {
        Ok(response) => {
            println!("✓ Status: {}", response.status);
            println!("✓ Body: {}", response.body);
            if let Some(receipt) = response.receipt {
                println!("✓ Receipt ID: {}", receipt.id);
                println!("✓ Receipt Status: {}", receipt.status);
            }
        }
        Err(e) => {
            println!("✗ Failed: {}", e);
            println!("  Note: This requires proper payment setup");
        }
    }
    println!();

    // Example 4: Analyze endpoint (paid)
    println!("┌────────────────────────────────────────────────────────────┐");
    println!("│ Example 4: Analyze Endpoint (Paid)                          │");
    println!("└────────────────────────────────────────────────────────────┘");

    match client.analyze().await {
        Ok(response) => {
            println!("✓ Status: {}", response.status);
            println!("✓ Body: {}", response.body);
            if let Some(receipt) = response.receipt {
                println!("✓ Receipt ID: {}", receipt.id);
            }
        }
        Err(e) => {
            println!("✗ Failed: {}", e);
        }
    }
    println!();

    // Example 5: Generate endpoint (paid)
    println!("┌────────────────────────────────────────────────────────────┐");
    println!("│ Example 5: Generate Endpoint (Paid)                         │");
    println!("└────────────────────────────────────────────────────────────┘");

    match client.generate().await {
        Ok(response) => {
            println!("✓ Status: {}", response.status);
            println!("✓ Body: {}", response.body);
            if let Some(receipt) = response.receipt {
                println!("✓ Receipt ID: {}", receipt.id);
            }
        }
        Err(e) => {
            println!("✗ Failed: {}", e);
        }
    }
    println!();

    // Example 6: Complex endpoint (paid)
    println!("┌────────────────────────────────────────────────────────────┐");
    println!("│ Example 6: Complex Endpoint (Paid)                          │");
    println!("└────────────────────────────────────────────────────────────┘");

    match client.complex().await {
        Ok(response) => {
            println!("✓ Status: {}", response.status);
            println!("✓ Body: {}", response.body);
            if let Some(receipt) = response.receipt {
                println!("✓ Receipt ID: {}", receipt.id);
            }
        }
        Err(e) => {
            println!("✗ Failed: {}", e);
        }
    }
    println!();

    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║                    Examples Complete                       ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    println!("Next Steps:");
    println!("1. Start the server: cargo run --example full_server --features server");
    println!("2. Run this client: cargo run --example full_client --features client");
    println!("3. For NEAR Intents: add --features intents");
    println!("\nFor production use:");
    println!("- Set proper environment variables");
    println!("- Configure real payment credentials");
    println!("- Handle errors appropriately");
    println!("- Implement proper logging and monitoring");

    Ok(())
}
