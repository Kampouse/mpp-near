//! Seamless Agent Client with auto-402 handling
//!
//! This client automatically handles HTTP 402 Payment Required responses,
//! making it easy for autonomous agents to interact with paid APIs.
//!
//! # Features
//!
//! - **Auto-402 detection**: Automatically detects payment-required responses
//! - **Gasless payments**: Uses OutLayer API for feeless transactions
//! - **Cross-chain support**: Bridge payments to Ethereum, Solana, Bitcoin, etc.
//! - **Budget controls**: Per-request and daily spending limits
//! - **Session caching**: Avoid re-paying for the same resource
//! - **Receipt caching**: Reuse payment proofs for identical challenges
//!
//! # Example
//!
//! ```rust,no_run
//! use mpp_near::client::{AgentClient, BudgetConfig};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create client with budget limits
//!     let client = AgentClient::new("wk_your_api_key")
//!         .with_budget(BudgetConfig::new(0.10, 5.0));
//!     
//!     // GET request - auto-handles 402 payment
//!     let data = client.get("https://paid-api.com/data").await?;
//!     
//!     // POST request - also auto-handles payment
//!     let result = client.post("https://api.example.com/submit", &serde_json::json!({"key": "value"})).await?;
//!     
//!     println!("Spent today: ${:.4}", client.spent_today());
//!     
//!     Ok(())
//! }
//! ```
//!
//! # Cross-Chain Payments
//!
//! The client automatically detects when payment is required on a different chain
//! and uses the MPP Bridge to forward payments:
//!
//! ```rust,no_run
//! # use mpp_near::client::AgentClient;
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = AgentClient::new("wk_your_api_key");
//!
//! // Server returns: 402 "Pay 1 USDC to 0xABC... on Ethereum"
//! // Client automatically bridges via NEAR Intents + OutLayer
//! let data = client.get("https://ethereum-mpp-api.com/data").await?;
//! # Ok(())
//! # }
//! ```

use reqwest::{Client, Response, header};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use base64::{Engine as _, engine::general_purpose};

/// Errors that can occur during agent client operations
#[derive(Debug, thiserror::Error)]
pub enum AgentError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("Invalid 402 challenge: {0}")]
    InvalidChallenge(String),
    
    #[error("Budget exceeded: requested ${requested:.4}, available ${available:.4}")]
    BudgetExceeded { requested: f64, available: f64 },
    
    #[error("Payment failed: {0}")]
    PaymentFailed(String),
    
    #[error("Failed to parse response: {0}")]
    ParseError(String),
    
    #[error("Challenge parsing error: {0}")]
    ChallengeParse(String),
    
    #[error("Session expired")]
    SessionExpired,
}

/// Budget configuration for the agent
/// 
/// Controls spending limits to prevent runaway costs when accessing paid APIs.
/// 
/// # Example
/// 
/// ```
/// use mpp_near::client::BudgetConfig;
/// 
/// let budget = BudgetConfig::new(0.10, 5.0); // $0.10 per request, $5.00 per day
/// 
/// assert!(budget.can_afford(0.05));
/// assert!(!budget.can_afford(0.20)); // Exceeds per-request limit
/// ```
#[derive(Clone, Debug)]
pub struct BudgetConfig {
    /// Maximum payment per request (in USD)
    pub max_per_request: f64,
    
    /// Maximum payment per day (in USD)
    pub max_per_day: f64,
    
    /// Amount spent today (in USD)
    pub spent_today: f64,
    
    /// Require manual approval for payments above this amount
    pub require_approval_above: f64,
    
    /// Last reset time (for daily budget)
    pub last_reset: Option<Instant>,
}

impl Default for BudgetConfig {
    fn default() -> Self {
        Self {
            max_per_request: 0.10,
            max_per_day: 5.00,
            spent_today: 0.0,
            require_approval_above: 0.50,
            last_reset: None,
        }
    }
}

impl BudgetConfig {
    /// Create a new budget config with custom limits
    /// 
    /// # Arguments
    /// 
    /// * `max_per_request` - Maximum payment per request in USD
    /// * `max_per_day` - Maximum total payment per day in USD
    /// 
    /// # Example
    /// 
    /// ```
    /// use mpp_near::client::BudgetConfig;
    /// 
    /// let budget = BudgetConfig::new(0.10, 5.0);
    /// ```
    pub fn new(max_per_request: f64, max_per_day: f64) -> Self {
        Self {
            max_per_request,
            max_per_day,
            ..Default::default()
        }
    }
    
    /// Get remaining budget for today
    pub fn remaining(&self) -> f64 {
        (self.max_per_day - self.spent_today).max(0.0)
    }
    
    /// Check if amount is within budget
    pub fn can_afford(&self, amount: f64) -> bool {
        amount <= self.max_per_request && amount <= self.remaining()
    }
    
    /// Record a payment
    pub fn record_payment(&mut self, amount: f64) {
        self.spent_today += amount;
    }
    
    /// Reset daily budget (call at midnight)
    pub fn reset_daily(&mut self) {
        self.spent_today = 0.0;
        self.last_reset = Some(Instant::now());
    }
}

/// Session cache entry
#[derive(Clone, Debug)]
struct Session {
    token: String,
    expires_at: Instant,
}

impl Session {
    fn new(token: String, duration: Duration) -> Self {
        Self {
            token,
            expires_at: Instant::now() + duration,
        }
    }
    
    fn expired(&self) -> bool {
        Instant::now() >= self.expires_at
    }
}

/// Payment receipt from OutLayer
#[derive(Serialize, Deserialize, Debug, Clone)]
struct PaymentReceipt {
    status: String,
    intent_hash: Option<String>,
    timestamp: Option<u64>,
    amount_out: Option<String>,
    /// NEAR transaction hash (for cross-chain payments)
    near_tx_hash: Option<String>,
    /// Target chain transaction hash (for cross-chain payments)
    target_tx_hash: Option<String>,
    /// Target chain name (for cross-chain payments)
    target_chain: Option<String>,
}

/// Token info for payment processing
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct TokenInfo {
    symbol: String,
    near_token_id: String,
    decimals: u8,
}

/// Cross-chain withdraw response from OutLayer
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct CrossChainResponse {
    near_tx: Option<String>,
    target_tx: String,
    status: String,
}

/// Parsed 402 challenge
#[derive(Debug, Clone)]
pub struct Challenge402 {
    pub realm: String,
    pub method: String,
    pub request: String,
    pub amount: f64,
    pub token: String,
    pub recipient: String,
    pub challenge: String,
    pub nonce: String,
}

impl Challenge402 {
    /// Parse WWW-Authenticate header
    pub fn parse(www_auth: &str) -> Result<Self, AgentError> {
        // Expected format:
        // NEAR-Intents realm="api.example.com", method="near-intents", 
        // request="0.001 USDC to merchant.near", challenge="abc123", nonce="xyz789"
        
        let mut params: HashMap<String, String> = HashMap::new();
        
        // Split by comma, but handle quoted values
        let mut current_key = String::new();
        let mut current_value = String::new();
        let mut in_quotes = false;
        let mut chars = www_auth.chars().peekable();
        
        // Skip scheme name (NEAR-Intents)
        while let Some(&c) = chars.peek() {
            if c == ' ' {
                chars.next();
                break;
            }
            chars.next();
        }
        
        for c in chars {
            match c {
                '=' if !in_quotes => {
                    current_key = current_value.trim().to_string();
                    current_value = String::new();
                }
                '"' => {
                    in_quotes = !in_quotes;
                }
                ',' if !in_quotes => {
                    if !current_key.is_empty() {
                        params.insert(current_key.clone(), current_value.trim().to_string());
                    }
                    current_key = String::new();
                    current_value = String::new();
                }
                _ => {
                    current_value.push(c);
                }
            }
        }
        
        // Don't forget the last param
        if !current_key.is_empty() {
            params.insert(current_key, current_value.trim().to_string());
        }
        
        let request = params.get("request")
            .ok_or_else(|| AgentError::ChallengeParse("Missing 'request' in challenge".into()))?
            .clone();
        
        // Parse request: "0.001 USDC to merchant.near"
        let (amount, token, recipient) = Self::parse_request(&request)?;
        
        Ok(Self {
            realm: params.get("realm").cloned().unwrap_or_default(),
            method: params.get("method").cloned().unwrap_or_else(|| "near-intents".into()),
            request: request.clone(),
            amount,
            token,
            recipient,
            challenge: params.get("challenge")
                .ok_or_else(|| AgentError::ChallengeParse("Missing 'challenge'".into()))?
                .clone(),
            nonce: params.get("nonce")
                .ok_or_else(|| AgentError::ChallengeParse("Missing 'nonce'".into()))?
                .clone(),
        })
    }
    
    /// Parse request string: "0.001 USDC to merchant.near"
    fn parse_request(request: &str) -> Result<(f64, String, String), AgentError> {
        // Try to parse: "<amount> <token> to <recipient>"
        let parts: Vec<&str> = request.split_whitespace().collect();
        
        if parts.len() < 4 || parts[2] != "to" {
            return Err(AgentError::ChallengeParse(format!("Invalid request format: {}", request)));
        }
        
        let amount: f64 = parts[0].parse()
            .map_err(|_| AgentError::ChallengeParse(format!("Invalid amount: {}", parts[0])))?;
        
        let token = parts[1].to_string();
        let recipient = parts[3..].join(" ");
        
        Ok((amount, token, recipient))
    }
}

/// Payment cache for sessions and receipts
#[derive(Default)]
struct PaymentCache {
    sessions: RwLock<HashMap<String, Session>>,
    receipts: RwLock<HashMap<String, PaymentReceipt>>,
}

impl PaymentCache {
    fn get_session(&self, url: &str) -> Option<Session> {
        self.sessions.read().ok()?.get(url).cloned()
    }
    
    fn store_session(&self, url: &str, token: String, duration: Duration) {
        if let Ok(mut sessions) = self.sessions.write() {
            sessions.insert(url.to_string(), Session::new(token, duration));
        }
    }
    
    fn get_receipt(&self, challenge: &str) -> Option<PaymentReceipt> {
        self.receipts.read().ok()?.get(challenge).cloned()
    }
    
    fn store_receipt(&self, challenge: &str, receipt: PaymentReceipt) {
        if let Ok(mut receipts) = self.receipts.write() {
            receipts.insert(challenge.to_string(), receipt);
        }
    }
}

/// Seamless agent client with auto-402 handling
/// 
/// This client automatically handles HTTP 402 Payment Required responses,
/// making it easy for autonomous agents to interact with paid APIs.
/// 
/// # Features
/// 
/// - Auto-detect and parse 402 challenges
/// - Pay via OutLayer API (gasless)
/// - Build and send payment credentials
/// - Retry requests with payment proof
/// - Session and receipt caching
/// 
/// # Example
/// 
/// ```rust,no_run
/// use mpp_near::client::{AgentClient, BudgetConfig};
/// 
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = AgentClient::new("wk_your_api_key")
///         .with_budget(BudgetConfig::new(0.10, 5.0));
///     
///     // This will auto-handle 402 if the API requires payment
///     let data = client.get("https://paid-api.com/data").await?;
///     
///     Ok(())
/// }
/// ```
pub struct AgentClient {
    http: Client,
    api_key: String,
    outlayer_url: String,
    budget: Arc<RwLock<BudgetConfig>>,
    cache: PaymentCache,
    cache_enabled: bool,
    auto_pay: bool,
}

impl AgentClient {
    /// Create a new agent client with OutLayer API key
    /// 
    /// # Arguments
    /// 
    /// * `api_key` - OutLayer API key (get from `mpp-near register`)
    /// 
    /// # Example
    /// 
    /// ```
    /// use mpp_near::client::AgentClient;
    /// 
    /// let client = AgentClient::new("wk_your_api_key");
    /// ```
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            http: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .unwrap_or_default(),
            api_key: api_key.into(),
            outlayer_url: "https://api.outlayer.fastnear.com".into(),
            budget: Arc::new(RwLock::new(BudgetConfig::default())),
            cache: PaymentCache::default(),
            cache_enabled: true,
            auto_pay: true,
        }
    }
    
    /// Set custom OutLayer API URL
    pub fn with_outlayer_url(mut self, url: impl Into<String>) -> Self {
        self.outlayer_url = url.into();
        self
    }
    
    /// Configure budget limits
    pub fn with_budget(mut self, config: BudgetConfig) -> Self {
        self.budget = Arc::new(RwLock::new(config));
        self
    }
    
    /// Enable or disable payment caching
    pub fn with_cache(mut self, enabled: bool) -> Self {
        self.cache_enabled = enabled;
        self
    }
    
    /// Enable or disable auto-payment (if false, returns error on 402)
    pub fn with_auto_pay(mut self, enabled: bool) -> Self {
        self.auto_pay = enabled;
        self
    }
    
    /// Get current budget status
    pub fn budget_status(&self) -> BudgetConfig {
        self.budget.read().unwrap().clone()
    }
    
    /// Get amount spent today
    pub fn spent_today(&self) -> f64 {
        self.budget.read().unwrap().spent_today
    }
    
    /// Get remaining budget
    pub fn remaining_budget(&self) -> f64 {
        self.budget.read().unwrap().remaining()
    }
    
    /// Clear payment cache
    pub fn clear_cache(&self) {
        if let Ok(mut sessions) = self.cache.sessions.write() {
            sessions.clear();
        }
        if let Ok(mut receipts) = self.cache.receipts.write() {
            receipts.clear();
        }
    }
    
    /// GET request with auto-402 handling
    /// 
    /// Automatically detects 402 responses, pays the challenge, and retries.
    /// 
    /// # Arguments
    /// 
    /// * `url` - URL to fetch
    /// 
    /// # Returns
    /// 
    /// HTTP response after handling any payment requirements
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// # use mpp_near::client::AgentClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AgentClient::new("wk_...");
    /// let resp = client.get("https://paid-api.com/data").await?;
    /// let json: serde_json::Value = resp.json().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, url: &str) -> Result<Response, AgentError> {
        self.request(reqwest::Method::GET, url, None::<&()>).await
    }
    
    /// POST request with auto-402 handling
    /// 
    /// Automatically detects 402 responses, pays the challenge, and retries.
    /// 
    /// # Arguments
    /// 
    /// * `url` - URL to post to
    /// * `body` - JSON body to send
    /// 
    /// # Returns
    /// 
    /// HTTP response after handling any payment requirements
    /// 
    /// # Example
    /// 
    /// ```rust,no_run
    /// # use mpp_near::client::AgentClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AgentClient::new("wk_...");
    /// let resp = client.post("https://api.example.com/submit", &serde_json::json!({
    ///     "key": "value"
    /// })).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn post<T: Serialize>(&self, url: &str, body: &T) -> Result<Response, AgentError> {
        self.request(reqwest::Method::POST, url, Some(body)).await
    }
    
    /// Generic request with auto-402 handling
    pub async fn request<T: Serialize>(
        &self,
        method: reqwest::Method,
        url: &str,
        body: Option<&T>,
    ) -> Result<Response, AgentError> {
        // Check session cache
        if self.cache_enabled && method == reqwest::Method::GET {
            if let Some(session) = self.cache.get_session(url) {
                if !session.expired() {
                    let resp = self.http
                        .request(method.clone(), url)
                        .header("Cookie", format!("session={}", session.token))
                        .send()
                        .await?;
                    
                    if !resp.status().is_client_error() {
                        return Ok(resp);
                    }
                }
            }
        }
        
        // Make initial request
        let mut req = self.http.request(method.clone(), url);
        if let Some(b) = body {
            req = req.json(b);
        }
        
        let resp = req.send().await?;
        
        // Handle 402 Payment Required
        if resp.status() == 402 {
            if !self.auto_pay {
                return Err(AgentError::PaymentFailed(
                    "Payment required but auto-pay is disabled".into()
                ));
            }
            
            self.handle_402(method, url, body).await
        } else {
            Ok(resp)
        }
    }
    
    /// Handle 402 payment flow
    async fn handle_402<T: Serialize>(
        &self,
        method: reqwest::Method,
        url: &str,
        body: Option<&T>,
    ) -> Result<Response, AgentError> {
        // Get WWW-Authenticate header
        let www_auth = self.http
            .request(method.clone(), url)
            .send()
            .await?
            .headers()
            .get(header::WWW_AUTHENTICATE)
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| AgentError::InvalidChallenge("No WWW-Authenticate header".into()))?
            .to_string();
        
        // Parse challenge
        let challenge = Challenge402::parse(&www_auth)?;
        
        // Check budget
        {
            let budget = self.budget.write().unwrap();
            if !budget.can_afford(challenge.amount) {
                return Err(AgentError::BudgetExceeded {
                    requested: challenge.amount,
                    available: budget.remaining(),
                });
            }
        }
        
        // Check receipt cache
        if self.cache_enabled {
            if let Some(receipt) = self.cache.get_receipt(&challenge.challenge) {
                // Try to reuse cached receipt
                if let Ok(resp) = self.retry_with_credential(method.clone(), url, body, &receipt, &challenge).await {
                    return Ok(resp);
                }
            }
        }
        
        // Pay via OutLayer
        let receipt = self.pay_challenge(&challenge).await?;
        
        // Record payment in budget
        {
            let mut budget = self.budget.write().unwrap();
            budget.record_payment(challenge.amount);
        }
        
        // Cache receipt
        if self.cache_enabled {
            self.cache.store_receipt(&challenge.challenge, receipt.clone());
        }
        
        // Retry with credential
        let resp = self.retry_with_credential(method, url, body, &receipt, &challenge).await?;
        
        // Cache session if provided
        if self.cache_enabled {
            if let Some(session_cookie) = resp.headers().get(header::SET_COOKIE) {
                if let Ok(cookie_str) = session_cookie.to_str() {
                    // Parse session token from cookie
                    if let Some(token) = cookie_str.split('=').nth(1) {
                        let token = token.split(';').next().unwrap_or("").to_string();
                        self.cache.store_session(url, token, Duration::from_secs(3600));
                    }
                }
            }
        }
        
        Ok(resp)
    }
    
    /// Pay challenge via OutLayer API
    /// 
    /// Automatically detects if recipient is on a different chain and uses
    /// cross-chain bridge if needed.
    async fn pay_challenge(&self, challenge: &Challenge402) -> Result<PaymentReceipt, AgentError> {
        // Check if this is a cross-chain payment
        if let Some(target_chain) = self.detect_cross_chain(&challenge.recipient) {
            tracing::info!("🌉 Cross-chain payment detected: {} → {}", challenge.recipient, target_chain);
            return self.pay_cross_chain(challenge, &target_chain).await;
        }
        
        // Native NEAR payment
        let token_id = self.get_token_id(&challenge.token);
        let amount_raw = self.amount_to_raw(challenge.amount, &challenge.token);
        
        let pay_request = serde_json::json!({
            "recipient": challenge.recipient,
            "amount": amount_raw.to_string(),
            "token": token_id,
            "memo": format!("challenge:{}", challenge.challenge)
        });
        
        let url = format!("{}/wallet/v1/intents/pay", self.outlayer_url);
        
        let resp = self.http
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&pay_request)
            .send()
            .await?;
        
        if !resp.status().is_success() {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".into());
            return Err(AgentError::PaymentFailed(error_text));
        }
        
        let mut receipt: PaymentReceipt = resp.json().await
            .map_err(|e| AgentError::ParseError(e.to_string()))?;
        
        if receipt.status != "success" {
            return Err(AgentError::PaymentFailed(format!(
                "Payment status: {}", receipt.status
            )));
        }
        
        // Ensure cross-chain fields are None for native payments
        receipt.near_tx_hash = None;
        receipt.target_tx_hash = None;
        receipt.target_chain = None;
        
        Ok(receipt)
    }
    
    /// Retry request with payment credential
    async fn retry_with_credential<T: Serialize>(
        &self,
        method: reqwest::Method,
        url: &str,
        body: Option<&T>,
        receipt: &PaymentReceipt,
        challenge: &Challenge402,
    ) -> Result<Response, AgentError> {
        let credential = self.build_credential(receipt, challenge);
        
        let mut req = self.http.request(method, url)
            .header("Authorization", format!("NEAR-Intents credential={}", credential));
        
        if let Some(b) = body {
            req = req.json(b);
        }
        
        Ok(req.send().await?)
    }
    
    /// Build base64 credential
    /// 
    /// For cross-chain payments, includes target chain tx hash instead of NEAR intent hash.
    fn build_credential(&self, receipt: &PaymentReceipt, challenge: &Challenge402) -> String {
        // For cross-chain payments, use target chain tx hash
        // This allows standard MPP servers to verify the payment normally
        let tx_hash = if let Some(ref target_tx) = receipt.target_tx_hash {
            target_tx.clone()
        } else {
            receipt.intent_hash.clone().unwrap_or_default()
        };
        
        let cred = serde_json::json!({
            "tx": tx_hash,
            "challenge": challenge.challenge,
            "nonce": challenge.nonce,
            "timestamp": receipt.timestamp.unwrap_or_else(|| {
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64
            }),
            // Include chain info for cross-chain payments
            "chain": receipt.target_chain.clone().unwrap_or_else(|| "near".into()),
        });
        
        general_purpose::URL_SAFE_NO_PAD.encode(cred.to_string())
    }
    
    /// Get token ID for OutLayer API
    fn get_token_id(&self, token: &str) -> String {
        match token.to_lowercase().as_str() {
            "near" => "near".into(),
            "usdc" => "nep141:17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1".into(),
            "usdt" => "nep141:usdt.tether-token.near".into(),
            _ if token.starts_with("nep141:") => token.into(),
            _ => format!("nep141:{}", token),
        }
    }
    
    /// Convert human-readable amount to raw amount
    fn amount_to_raw(&self, amount: f64, token: &str) -> u128 {
        let decimals = match token.to_lowercase().as_str() {
            "near" => 24,
            "usdc" | "usdt" => 6,
            _ => 24, // Default to NEAR decimals
        };
        
        (amount * 10f64.powi(decimals as i32)) as u128
    }
    
    /// Detect if recipient is on a different chain
    /// 
    /// Returns Some(chain) if the recipient address format matches a non-NEAR chain
    fn detect_cross_chain(&self, recipient: &str) -> Option<String> {
        // Ethereum-like (0x prefix, 42 chars)
        if recipient.starts_with("0x") && recipient.len() == 42 {
            return Some("ethereum".into());
        }
        
        // Bitcoin (1, 3, or bc1 prefix)
        if recipient.starts_with("1") || recipient.starts_with("3") || recipient.starts_with("bc1") {
            return Some("bitcoin".into());
        }
        
        // Solana (Base58, 32-44 chars, alphanumeric without 0, O, I, l)
        if recipient.len() >= 32 && recipient.len() <= 44 {
            let base58_chars = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
            if recipient.chars().all(|c| base58_chars.contains(c)) {
                return Some("solana".into());
            }
        }
        
        // NEAR (.near or .testnet suffix, or 64-char hex)
        if recipient.ends_with(".near") || recipient.ends_with(".testnet") ||
           (recipient.len() == 64 && recipient.chars().all(|c| c.is_ascii_hexdigit())) {
            return None; // Native NEAR, no bridging needed
        }
        
        None
    }
    
    /// Pay via cross-chain bridge
    /// 
    /// Used when the recipient is on a different chain (Ethereum, Solana, Bitcoin, etc.)
    async fn pay_cross_chain(&self, challenge: &Challenge402, target_chain: &str) -> Result<PaymentReceipt, AgentError> {
        let token_info = self.get_token_info(&challenge.token);
        let amount_raw = (challenge.amount * 10f64.powi(token_info.decimals as i32)) as u64;
        
        // Call OutLayer cross-chain withdraw directly
        let url = format!("{}/wallet/v1/intents/withdraw", self.outlayer_url);
        
        let withdraw_request = serde_json::json!({
            "to": challenge.recipient,
            "amount": amount_raw.to_string(),
            "token": token_info.near_token_id,
            "chain": target_chain,
        });
        
        let resp = self.http
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&withdraw_request)
            .send()
            .await?;
        
        if !resp.status().is_success() {
            let error_text = resp.text().await.unwrap_or_else(|_| "Unknown error".into());
            return Err(AgentError::PaymentFailed(format!(
                "Cross-chain withdraw failed: {}", error_text
            )));
        }
        
        let withdraw_response: CrossChainResponse = resp.json().await
            .map_err(|e| AgentError::ParseError(e.to_string()))?;
        
        // Build receipt with target chain tx hash
        Ok(PaymentReceipt {
            status: "success".into(),
            intent_hash: Some(withdraw_response.target_tx.clone()),
            near_tx_hash: withdraw_response.near_tx,
            target_tx_hash: Some(withdraw_response.target_tx),
            target_chain: Some(target_chain.into()),
            timestamp: Some(SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64),
            amount_out: None,
        })
    }
    
    /// Get token info (symbol, token_id, decimals)
    fn get_token_info(&self, token: &str) -> TokenInfo {
        match token.to_lowercase().as_str() {
            "usdc" => TokenInfo {
                symbol: "USDC".into(),
                near_token_id: "17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1".into(),
                decimals: 6,
            },
            "usdt" => TokenInfo {
                symbol: "USDT".into(),
                near_token_id: "usdt.tether-token.near".into(),
                decimals: 6,
            },
            "near" => TokenInfo {
                symbol: "NEAR".into(),
                near_token_id: "near".into(),
                decimals: 24,
            },
            _ => TokenInfo {
                symbol: token.into(),
                near_token_id: token.into(),
                decimals: 24,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_budget_config() {
        let budget = BudgetConfig::new(0.10, 5.0);
        assert!(budget.can_afford(0.05));
        assert!(budget.can_afford(0.10));
        assert!(!budget.can_afford(0.15));
    }
    
    #[test]
    fn test_challenge_parse() {
        let www_auth = r#"NEAR-Intents realm="api.example.com", method="near-intents", request="0.001 USDC to merchant.near", challenge="abc123", nonce="xyz789""#;
        
        let challenge = Challenge402::parse(www_auth).unwrap();
        assert_eq!(challenge.amount, 0.001);
        assert_eq!(challenge.token, "USDC");
        assert_eq!(challenge.recipient, "merchant.near");
        assert_eq!(challenge.challenge, "abc123");
        assert_eq!(challenge.nonce, "xyz789");
    }
    
    #[test]
    fn test_budget_remaining() {
        let mut budget = BudgetConfig::default();
        budget.spent_today = 2.0;
        assert_eq!(budget.remaining(), 3.0);
        
        budget.record_payment(1.5);
        assert_eq!(budget.spent_today, 3.5);
        assert_eq!(budget.remaining(), 1.5);
    }
}
