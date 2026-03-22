//! Bridge client for cross-chain MPP payments

use crate::bridge::types::{BridgeError, BridgeRequest, BridgeResponse, BridgeStatus, Chain, TokenInfo};
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

/// Bridge client for making cross-chain payments
/// 
/// This client handles the full bridge flow:
/// 1. Pay to bridge via NEAR Intents (gasless)
/// 2. Bridge forwards to target chain via OutLayer
/// 3. Returns native chain tx hash for MPP credential
#[derive(Debug)]
pub struct BridgeClient {
    http: Client,
    bridge_url: String,
    outlayer_api_key: String,
    outlayer_url: String,
}

impl BridgeClient {
    /// Create a new bridge client
    /// 
    /// # Arguments
    /// 
    /// * `bridge_url` - URL of the MPP bridge service
    /// * `outlayer_api_key` - OutLayer API key for payments
    pub fn new(bridge_url: impl Into<String>, outlayer_api_key: impl Into<String>) -> Self {
        Self {
            http: Client::builder()
                .timeout(Duration::from_secs(60))
                .build()
                .unwrap_or_default(),
            bridge_url: bridge_url.into(),
            outlayer_api_key: outlayer_api_key.into(),
            outlayer_url: "https://api.outlayer.fastnear.com".into(),
        }
    }
    
    /// Set custom OutLayer URL
    pub fn with_outlayer_url(mut self, url: impl Into<String>) -> Self {
        self.outlayer_url = url.into();
        self
    }
    
    /// Pay and bridge in one call
    /// 
    /// This method:
    /// 1. Pays the bridge via NEAR Intents
    /// 2. Waits for cross-chain confirmation
    /// 3. Returns target chain tx hash
    /// 
    /// # Arguments
    /// 
    /// * `request` - Bridge payment request
    /// 
    /// # Returns
    /// 
    /// Bridge response with target chain tx hash
    pub async fn pay_and_bridge(&self, request: BridgeRequest) -> Result<BridgeResponse, BridgeError> {
        // Validate request
        self.validate_request(&request)?;
        
        // Call bridge service
        let url = format!("{}/bridge", self.bridge_url);
        
        let resp = self.http
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.outlayer_api_key))
            .json(&request)
            .send()
            .await
            .map_err(|e| BridgeError {
                error: "http_error".into(),
                message: e.to_string(),
                code: None,
            })?;
        
        if !resp.status().is_success() {
            let error = resp.json::<BridgeError>().await.unwrap_or_else(|_| BridgeError {
                error: "unknown_error".into(),
                message: "Unknown error from bridge".into(),
                code: None,
            });
            return Err(error);
        }
        
        let bridge_response: BridgeResponse = resp.json().await.map_err(|e| BridgeError {
            error: "parse_error".into(),
            message: e.to_string(),
            code: None,
        })?;
        
        Ok(bridge_response)
    }
    
    /// Pay directly via OutLayer cross-chain (no bridge service needed)
    /// 
    /// This method uses OutLayer's cross-chain withdraw directly.
    /// Use this when you don't have a bridge service running.
    /// 
    /// # Arguments
    /// 
    /// * `request` - Bridge payment request
    /// 
    /// # Returns
    /// 
    /// Bridge response with target chain tx hash
    pub async fn pay_direct(&self, request: BridgeRequest) -> Result<BridgeResponse, BridgeError> {
        // Validate request
        self.validate_request(&request)?;
        
        let token_info = TokenInfo::from_symbol(&request.token)
            .ok_or_else(|| BridgeError {
                error: "invalid_token".into(),
                message: format!("Unsupported token: {}", request.token),
                code: Some(400),
            })?;
        
        // Convert amount to raw
        let amount_raw = (request.amount * 10f64.powi(token_info.decimals as i32)) as u64;
        
        // Call OutLayer cross-chain withdraw
        let url = format!("{}/wallet/v1/intents/withdraw", self.outlayer_url);
        
        let withdraw_request = serde_json::json!({
            "to": request.recipient,
            "amount": amount_raw.to_string(),
            "token": token_info.near_token_id,
            "chain": request.target_chain,
        });
        
        let resp = self.http
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.outlayer_api_key))
            .json(&withdraw_request)
            .send()
            .await
            .map_err(|e| BridgeError {
                error: "http_error".into(),
                message: e.to_string(),
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
        
        Ok(BridgeResponse {
            near_tx: withdraw_response.near_tx.unwrap_or_default(),
            target_tx: withdraw_response.target_tx,
            target_chain: request.target_chain.clone(),
            status: BridgeStatus::Submitted,
            amount: request.amount,
            token: request.token,
            timestamp: chrono::Utc::now().timestamp(),
            estimated_confirmation_ms: Some(30_000), // ~30s for most chains
        })
    }
    
    /// Check payment status
    pub async fn check_status(&self, near_tx: &str) -> Result<BridgeResponse, BridgeError> {
        let url = format!("{}/bridge/status/{}", self.bridge_url, near_tx);
        
        let resp = self.http
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.outlayer_api_key))
            .send()
            .await
            .map_err(|e| BridgeError {
                error: "http_error".into(),
                message: e.to_string(),
                code: None,
            })?;
        
        if !resp.status().is_success() {
            let error = resp.json::<BridgeError>().await.unwrap_or_else(|_| BridgeError {
                error: "unknown_error".into(),
                message: "Unknown error from bridge".into(),
                code: None,
            });
            return Err(error);
        }
        
        resp.json().await.map_err(|e| BridgeError {
            error: "parse_error".into(),
            message: e.to_string(),
            code: None,
        })
    }
    
    /// Validate bridge request
    fn validate_request(&self, request: &BridgeRequest) -> Result<(), BridgeError> {
        // Validate chain
        let chain = Chain::from_str(&request.target_chain)
            .ok_or_else(|| BridgeError {
                error: "invalid_chain".into(),
                message: format!("Unsupported chain: {}", request.target_chain),
                code: Some(400),
            })?;
        
        // Validate address format
        if !chain.is_valid_address(&request.recipient) {
            return Err(BridgeError {
                error: "invalid_address".into(),
                message: format!("Invalid {} address format", chain.as_str()),
                code: Some(400),
            });
        }
        
        // Validate token
        if TokenInfo::from_symbol(&request.token).is_none() {
            return Err(BridgeError {
                error: "invalid_token".into(),
                message: format!("Unsupported token: {}", request.token),
                code: Some(400),
            });
        }
        
        // Validate amount
        if request.amount <= 0.0 {
            return Err(BridgeError {
                error: "invalid_amount".into(),
                message: "Amount must be positive".into(),
                code: Some(400),
            });
        }
        
        Ok(())
    }
}

/// OutLayer withdraw API response
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct OutLayerWithdrawResponse {
    /// NEAR transaction hash (if available)
    near_tx: Option<String>,
    
    /// Target chain transaction hash
    target_tx: String,
    
    /// Status
    status: String,
}

/// Detect chain from address format
#[allow(dead_code)]
pub fn detect_chain(address: &str) -> Option<Chain> {
    // Ethereum-like (0x prefix, 42 chars)
    if address.starts_with("0x") && address.len() == 42 {
        return Some(Chain::Ethereum);
    }
    
    // Bitcoin (1, 3, or bc1 prefix)
    if address.starts_with("1") || address.starts_with("3") || address.starts_with("bc1") {
        return Some(Chain::Bitcoin);
    }
    
    // NEAR (.near or .testnet suffix, or 64-char hex)
    if address.ends_with(".near") || address.ends_with(".testnet") ||
       (address.len() == 64 && address.chars().all(|c| c.is_ascii_hexdigit())) {
        return Some(Chain::Near);
    }
    
    // Solana (Base58, 32-44 chars)
    if address.len() >= 32 && address.len() <= 44 {
        // Could be Solana, but hard to distinguish from other Base58
        // This is a heuristic
        if address.chars().all(|c| "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".contains(c)) {
            return Some(Chain::Solana);
        }
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_detect_chain_ethereum() {
        let addr = "0x742d35Cc6634C0532925a3b844Bc4591c494Bc4E";
        assert_eq!(detect_chain(addr), Some(Chain::Ethereum));
    }
    
    #[test]
    fn test_detect_chain_bitcoin() {
        let addr = "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh";
        assert_eq!(detect_chain(addr), Some(Chain::Bitcoin));
    }
    
    #[test]
    fn test_detect_chain_near() {
        let addr = "kampouse.near";
        assert_eq!(detect_chain(addr), Some(Chain::Near));
    }
    
    #[test]
    fn test_chain_validation() {
        let eth = Chain::Ethereum;
        assert!(eth.is_valid_address("0x742d35Cc6634C0532925a3b844Bc4591c494Bc4E"));
        assert!(!eth.is_valid_address("kampouse.near"));
        
        let near = Chain::Near;
        assert!(near.is_valid_address("kampouse.near"));
        assert!(!near.is_valid_address("0x742d35Cc6634C0532925a3b844Bc4591c494Bc4E"));
    }
}
