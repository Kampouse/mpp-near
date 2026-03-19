//! Gasless payment provider via NEAR Intents (OutLayer custody wallet)

use crate::types::{AccountId, NearAmount, NearChallenge, NearCredential, TransactionHash};
use crate::{Error, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

/// OutLayer custody wallet configuration
#[derive(Debug, Clone)]
pub struct IntentsConfig {
    /// OutLayer API base URL
    pub api_url: String,
    /// Wallet API key (wk_...)
    pub api_key: String,
    /// Default chain for operations
    pub default_chain: String,
    /// Cache duration for balance checks (seconds)
    pub balance_cache_ttl: u64,
}

impl Default for IntentsConfig {
    fn default() -> Self {
        Self {
            api_url: "https://api.outlayer.fastnear.com".to_string(),
            api_key: String::new(),
            default_chain: "near".to_string(),
            balance_cache_ttl: 30,
        }
    }
}

/// Provider state
#[derive(Debug, Default)]
struct ProviderState {
    near_account_id: Option<String>,
    cached_balance: Option<NearAmount>,
    balance_updated_at: Option<std::time::Instant>,
}

/// Gasless payment provider via NEAR Intents
pub struct IntentsProvider {
    config: IntentsConfig,
    client: Client,
    state: Arc<RwLock<ProviderState>>,
}

impl IntentsProvider {
    /// Create new Intents provider with API key
    pub fn new(api_key: String) -> Self {
        Self {
            config: IntentsConfig {
                api_key,
                ..Default::default()
            },
            client: Client::new(),
            state: Arc::new(RwLock::new(ProviderState::default())),
        }
    }
    
    /// Create with custom configuration
    pub fn with_config(config: IntentsConfig) -> Self {
        Self {
            config,
            client: Client::new(),
            state: Arc::new(RwLock::new(ProviderState::default())),
        }
    }
    
    /// Get NEAR account ID
    pub async fn get_account_id(&self) -> Result<String> {
        // Check cache
        {
            let state = self.state.read().await;
            if let Some(account_id) = &state.near_account_id {
                return Ok(account_id.clone());
            }
        }
        
        // Fetch from API
        let response = self.client
            .get(&format!("{}/wallet/v1/address?chain=near", self.config.api_url))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .send()
            .await?
            .json::<AddressResponse>()
            .await?;
        
        let account_id = response.address;
        
        // Update cache
        {
            let mut state = self.state.write().await;
            state.near_account_id = Some(account_id.clone());
        }
        
        Ok(account_id)
    }
    
    /// Check NEAR balance (for gas)
    pub async fn check_balance(&self) -> Result<NearAmount> {
        // Check cache
        {
            let state = self.state.read().await;
            if let (Some(balance), Some(updated_at)) = 
                (state.cached_balance, state.balance_updated_at) {
                let elapsed = updated_at.elapsed().as_secs();
                if elapsed < self.config.balance_cache_ttl {
                    debug!("Using cached balance: {}", balance);
                    return Ok(balance);
                }
            }
        }
        
        // Fetch from API
        let response = self.client
            .get(&format!("{}/wallet/v1/balance?chain=near", self.config.api_url))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .send()
            .await?
            .json::<BalanceResponse>()
            .await?;
        
        let balance = NearAmount::from_yocto(response.balance.parse().unwrap_or(0));
        
        // Update cache
        {
            let mut state = self.state.write().await;
            state.cached_balance = Some(balance);
            state.balance_updated_at = Some(std::time::Instant::now());
        }
        
        info!("NEAR balance: {}", balance);
        Ok(balance)
    }
    
    /// Check intents balance (for gasless operations)
    pub async fn check_intents_balance(&self, token: &str) -> Result<NearAmount> {
        let response = self.client
            .get(&format!(
                "{}/wallet/v1/balance?token={}&source=intents",
                self.config.api_url,
                token
            ))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .send()
            .await?
            .json::<BalanceResponse>()
            .await?;
        
        let balance = NearAmount::from_yocto(response.balance.parse().unwrap_or(0));
        
        info!("Intents balance for {}: {}", token, balance);
        Ok(balance)
    }
    
    /// Gasless transfer via NEAR Intents
    pub async fn transfer(&self, recipient: &AccountId, amount: NearAmount) -> Result<TransactionHash> {
        info!("Gasless transfer of {} to {}", amount, recipient);
        
        let request = WithdrawRequest {
            to: recipient.as_str().to_string(),
            amount: amount.0.to_string(),
            token: "near".to_string(),
            chain: self.config.default_chain.clone(),
        };
        
        let response = self.client
            .post(&format!("{}/wallet/v1/intents/withdraw", self.config.api_url))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .json(&request)
            .send()
            .await?
            .json::<WithdrawResponse>()
            .await?;
        
        match response.status.as_str() {
            "success" => {
                info!("Transfer successful: {}", response.intent_hash);
                TransactionHash::new(format!("0x{}", response.intent_hash))
            }
            _ => Err(Error::TransactionFailed(format!(
                "Transfer failed: {:?}",
                response
            ))),
        }
    }
    
    /// Gasless token transfer via NEAR Intents
    pub async fn transfer_token(
        &self,
        token: &str,
        recipient: &AccountId,
        amount: NearAmount,
    ) -> Result<TransactionHash> {
        info!("Gasless token transfer of {} to {}", amount, recipient);
        
        let request = WithdrawRequest {
            to: recipient.as_str().to_string(),
            amount: amount.0.to_string(),
            token: token.to_string(),
            chain: self.config.default_chain.clone(),
        };
        
        let response = self.client
            .post(&format!("{}/wallet/v1/intents/withdraw", self.config.api_url))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .json(&request)
            .send()
            .await?
            .json::<WithdrawResponse>()
            .await?;
        
        match response.status.as_str() {
            "success" => {
                info!("Token transfer successful: {}", response.intent_hash);
                TransactionHash::new(format!("0x{}", response.intent_hash))
            }
            _ => Err(Error::TransactionFailed(format!(
                "Token transfer failed: {:?}",
                response
            ))),
        }
    }
    
    /// Pay for a challenge (gasless)
    pub async fn pay_challenge(&self, challenge: &NearChallenge) -> Result<NearCredential> {
        debug!("Processing challenge (gasless): {}", challenge.challenge_id);
        
        // Validate expiration
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        
        if challenge.expires_at < now {
            return Err(Error::InvalidChallenge("Challenge expired".to_string()));
        }
        
        // Execute payment (gasless via Intents)
        let tx_hash = match challenge.method.as_str() {
            "near" => self.transfer(&challenge.recipient, challenge.amount).await?,
            "usdc" => {
                // USDC token ID on NEAR
                let usdc_token = "17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1";
                self.transfer_token(usdc_token, &challenge.recipient, challenge.amount).await?
            }
            "usdt" => {
                // USDT token ID on NEAR
                let usdt_token = "usdt.tether-token.near";
                self.transfer_token(usdt_token, &challenge.recipient, challenge.amount).await?
            }
            _ => return Err(Error::InvalidChallenge(format!("Unsupported method: {}", challenge.method))),
        };
        
        // Get block height (simplified)
        let block_height = self.get_block_height().await?;
        
        // Get account ID
        let payer = self.get_account_id().await?;
        
        Ok(NearCredential {
            tx_hash,
            payer: AccountId::new(&payer)?,
            recipient: challenge.recipient.clone(),
            amount: challenge.amount,
            block_height,
            signature: "intents_gasless".to_string(), // No signature needed for Intents
            timestamp: now,
        })
    }
    
    /// Get current block height
    async fn get_block_height(&self) -> Result<u64> {
        // Simplified - would query NEAR RPC
        Ok(123456789)
    }
    
    /// Create a payment check (agent-to-agent payment)
    pub async fn create_payment_check(
        &self,
        token: &str,
        amount: NearAmount,
        memo: Option<&str>,
        expires_in: Option<u64>,
    ) -> Result<PaymentCheck> {
        let request = CreateCheckRequest {
            token: token.to_string(),
            amount: amount.0.to_string(),
            memo: memo.map(|s| s.to_string()),
            expires_in,
        };
        
        let response = self.client
            .post(&format!("{}/wallet/v1/payment-check/create", self.config.api_url))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .json(&request)
            .send()
            .await?
            .json::<PaymentCheckResponse>()
            .await?;
        
        info!("Created payment check: {}", response.check_id);
        
        Ok(PaymentCheck {
            check_id: response.check_id,
            check_key: response.check_key,
            token: response.token,
            amount: NearAmount::from_yocto(response.amount.parse().unwrap_or(0)),
            memo: response.memo,
            expires_at: response.expires_at,
        })
    }
    
    /// Claim a payment check
    pub async fn claim_payment_check(&self, check_key: &str, amount: Option<NearAmount>) -> Result<NearAmount> {
        let request = ClaimCheckRequest {
            check_key: check_key.to_string(),
            amount: amount.map(|a| a.0.to_string()),
        };
        
        let response = self.client
            .post(&format!("{}/wallet/v1/payment-check/claim", self.config.api_url))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .json(&request)
            .send()
            .await?
            .json::<ClaimCheckResponse>()
            .await?;
        
        let claimed = NearAmount::from_yocto(response.amount_claimed.parse().unwrap_or(0));
        
        info!("Claimed payment check: {}", claimed);
        Ok(claimed)
    }
    
    /// Get available tokens for swaps
    pub async fn list_tokens(&self) -> Result<Vec<TokenInfo>> {
        let response = self.client
            .get(&format!("{}/wallet/v1/tokens", self.config.api_url))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .send()
            .await?
            .json::<TokensResponse>()
            .await?;
        
        Ok(response.tokens)
    }
    
    /// Swap tokens (gasless)
    pub async fn swap(
        &self,
        token_in: &str,
        token_out: &str,
        amount_in: NearAmount,
        min_amount_out: Option<NearAmount>,
    ) -> Result<SwapResult> {
        let request = SwapRequest {
            token_in: token_in.to_string(),
            token_out: token_out.to_string(),
            amount_in: amount_in.0.to_string(),
            min_amount_out: min_amount_out.map(|a| a.0.to_string()),
        };
        
        let response = self.client
            .post(&format!("{}/wallet/v1/intents/swap", self.config.api_url))
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .json(&request)
            .send()
            .await?
            .json::<SwapResponse>()
            .await?;
        
        match response.status.as_str() {
            "success" => {
                info!("Swap successful: {} → {}", token_in, token_out);
                Ok(SwapResult {
                    request_id: response.request_id,
                    amount_out: NearAmount::from_yocto(response.amount_out.unwrap_or_default().parse().unwrap_or(0)),
                    intent_hash: response.intent_hash,
                })
            }
            _ => Err(Error::TransactionFailed(format!(
                "Swap failed: {:?}",
                response
            ))),
        }
    }
}

// API response types

#[derive(Debug, Deserialize)]
struct AddressResponse {
    address: String,
}

#[derive(Debug, Deserialize)]
struct BalanceResponse {
    balance: String,
    #[allow(dead_code)]
    token: Option<String>,
    #[allow(dead_code)]
    account_id: Option<String>,
}

#[derive(Debug, Serialize)]
struct WithdrawRequest {
    to: String,
    amount: String,
    token: String,
    chain: String,
}

#[derive(Debug, Deserialize)]
struct WithdrawResponse {
    #[allow(dead_code)]
    request_id: String,
    status: String,
    intent_hash: String,
}

#[derive(Debug, Serialize)]
struct CreateCheckRequest {
    token: String,
    amount: String,
    memo: Option<String>,
    expires_in: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct PaymentCheckResponse {
    check_id: String,
    check_key: String,
    token: String,
    amount: String,
    memo: Option<String>,
    expires_at: Option<String>,
}

#[derive(Debug, Serialize)]
struct ClaimCheckRequest {
    check_key: String,
    amount: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ClaimCheckResponse {
    amount_claimed: String,
    #[allow(dead_code)]
    remaining: String,
}

#[derive(Debug, Deserialize)]
struct TokensResponse {
    tokens: Vec<TokenInfo>,
}

#[derive(Debug, Serialize)]
struct SwapRequest {
    token_in: String,
    token_out: String,
    amount_in: String,
    min_amount_out: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SwapResponse {
    request_id: String,
    status: String,
    amount_out: Option<String>,
    intent_hash: Option<String>,
}

/// Payment check
#[derive(Debug, Clone)]
pub struct PaymentCheck {
    pub check_id: String,
    pub check_key: String,
    pub token: String,
    pub amount: NearAmount,
    pub memo: Option<String>,
    pub expires_at: Option<String>,
}

/// Token info
#[derive(Debug, Clone, Deserialize)]
pub struct TokenInfo {
    pub symbol: String,
    pub name: String,
    pub chain: String,
    pub defuse_asset_id: String,
    pub decimals: u8,
}

/// Swap result
#[derive(Debug, Clone)]
pub struct SwapResult {
    pub request_id: String,
    pub amount_out: NearAmount,
    pub intent_hash: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_default() {
        let config = IntentsConfig::default();
        assert_eq!(config.api_url, "https://api.outlayer.fastnear.com");
        assert_eq!(config.default_chain, "near");
    }
    
    #[test]
    fn test_provider_creation() {
        let provider = IntentsProvider::new("wk_test123".to_string());
        assert_eq!(provider.config.api_key, "wk_test123");
    }
}
