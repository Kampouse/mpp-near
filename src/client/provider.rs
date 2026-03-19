//! NEAR payment provider implementation

use near_jsonrpc_client::JsonRpcClient;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

use crate::types::{AccountId, Gas, NearAmount, NearChallenge, NearCredential, TransactionHash};
use crate::{Error, Result};
use super::signer::NearSigner;

/// Configuration for NEAR payment provider
#[derive(Debug, Clone)]
pub struct NearConfig {
    /// NEAR RPC endpoint
    pub rpc_url: String,
    /// Account ID making payments
    pub account_id: AccountId,
    /// Gas to use for payments (default: 100 Tgas)
    pub gas: Gas,
    /// Maximum amount per payment (safety limit)
    pub max_amount: NearAmount,
    /// Network ("mainnet" or "testnet")
    pub network: String,
    /// Cache duration for balance checks (seconds)
    pub balance_cache_ttl: u64,
}

impl Default for NearConfig {
    fn default() -> Self {
        Self {
            rpc_url: "https://rpc.mainnet.near.org".to_string(),
            account_id: AccountId::new("anonymous.near").unwrap(),
            gas: Gas::DEFAULT,
            max_amount: NearAmount::from_near(100), // 100 NEAR max
            network: "mainnet".to_string(),
            balance_cache_ttl: 30,
        }
    }
}

/// Payment provider state
#[derive(Debug, Default)]
struct ProviderState {
    cached_balance: Option<NearAmount>,
    balance_updated_at: Option<std::time::Instant>,
}

/// NEAR payment provider for MPP
pub struct NearProvider {
    config: NearConfig,
    signer: NearSigner,
    client: JsonRpcClient,
    state: Arc<RwLock<ProviderState>>,
}

impl NearProvider {
    /// Create a new NEAR payment provider
    pub fn new(account_id: AccountId, private_key: String, rpc_url: &str) -> Result<Self> {
        let config = NearConfig {
            rpc_url: rpc_url.to_string(),
            account_id: account_id.clone(),
            ..Default::default()
        };
        
        let signer = NearSigner::new(account_id, private_key)?;
        let client = JsonRpcClient::connect(&config.rpc_url);
        
        Ok(Self {
            config,
            signer,
            client,
            state: Arc::new(RwLock::new(ProviderState::default())),
        })
    }
    
    /// Create with custom configuration
    pub fn with_config(config: NearConfig, private_key: String) -> Result<Self> {
        let signer = NearSigner::new(config.account_id.clone(), private_key)?;
        let client = JsonRpcClient::connect(&config.rpc_url);
        
        Ok(Self {
            config,
            signer,
            client,
            state: Arc::new(RwLock::new(ProviderState::default())),
        })
    }
    
    /// Check account balance (with caching)
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
        
        // Simplified - in production would query RPC
        // For now, return a placeholder balance
        let balance = NearAmount::from_near(10);
        
        // Update cache
        {
            let mut state = self.state.write().await;
            state.cached_balance = Some(balance);
            state.balance_updated_at = Some(std::time::Instant::now());
        }
        
        info!("Balance: {}", balance);
        Ok(balance)
    }
    
    /// Execute NEAR transfer
    pub async fn transfer(&self, recipient: &AccountId, amount: NearAmount) -> Result<TransactionHash> {
        // Safety check
        if amount.0 > self.config.max_amount.0 {
            return Err(Error::InsufficientBalance {
                required: amount.to_string(),
                available: self.config.max_amount.to_string(),
            });
        }
        
        // Check balance
        let balance = self.check_balance().await?;
        if balance.0 < amount.0 {
            return Err(Error::InsufficientBalance {
                required: amount.to_string(),
                available: balance.to_string(),
            });
        }
        
        info!("Transferring {} to {}", amount, recipient);
        
        // Build transaction (simplified - in production would use near-primitives)
        let mock_hash = format!("0x{}", hex::encode(&[0u8; 32]));
        TransactionHash::new(mock_hash)
    }
    
    /// Execute NEP-141 token transfer (USDC, etc.)
    pub async fn transfer_token(
        &self,
        _token_contract: &AccountId,
        _recipient: &AccountId,
        _amount: NearAmount,
    ) -> Result<TransactionHash> {
        // Simplified - would use function call in production
        let mock_hash = format!("0x{}", hex::encode(&[1u8; 32]));
        TransactionHash::new(mock_hash)
    }
    
    /// Pay for a challenge
    pub async fn pay_challenge(&self, challenge: &NearChallenge) -> Result<NearCredential> {
        debug!("Processing challenge: {}", challenge.challenge_id);
        
        // Validate expiration
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        
        if challenge.expires_at < now {
            return Err(Error::InvalidChallenge("Challenge expired".to_string()));
        }
        
        // Execute payment
        let tx_hash = match challenge.method.as_str() {
            "near" => self.transfer(&challenge.recipient, challenge.amount).await?,
            "usdc" => {
                let usdc_contract = AccountId::new("usdt.tether-token.near")
                    .map_err(|e| Error::InvalidAccountId(e.to_string()))?;
                self.transfer_token(&usdc_contract, &challenge.recipient, challenge.amount).await?
            }
            _ => return Err(Error::InvalidChallenge(format!("Unsupported method: {}", challenge.method))),
        };
        
        // Sign challenge
        let signature = self.signer.sign_challenge(&challenge.challenge_id)?;
        
        // Get block height (simplified)
        let block_height = self.get_block_height().await?;
        
        Ok(NearCredential {
            tx_hash,
            payer: self.config.account_id.clone(),
            recipient: challenge.recipient.clone(),
            amount: challenge.amount,
            block_height,
            signature,
            timestamp: now,
        })
    }
    
    /// Get current block height
    async fn get_block_height(&self) -> Result<u64> {
        // Simplified - in production would query RPC
        Ok(123456789)
    }
    
    /// Get account ID
    pub fn account_id(&self) -> &AccountId {
        &self.config.account_id
    }
}
