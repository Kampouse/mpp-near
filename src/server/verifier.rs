//! NEAR payment verification

use near_jsonrpc_client::JsonRpcClient;
use near_primitives::views::FinalExecutionStatus;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;
use tracing::{debug, info};

use crate::types::{AccountId, NearAmount, NearChallenge, NearCredential, TransactionHash};
use crate::{Error, Result};

/// Verifier configuration
#[derive(Debug, Clone)]
pub struct VerifierConfig {
    /// RPC endpoint
    pub rpc_url: String,
    /// Expected recipient account
    pub recipient_account: AccountId,
    /// Minimum payment amount
    pub min_amount: NearAmount,
    /// Challenge expiration time (seconds)
    pub challenge_ttl: u64,
    /// Transaction confirmation blocks
    pub confirmations: u64,
    /// Cache verified credentials
    pub cache_ttl: u64,
}

impl Default for VerifierConfig {
    fn default() -> Self {
        Self {
            rpc_url: "https://rpc.mainnet.near.org".to_string(),
            recipient_account: AccountId::new("merchant.near").unwrap(),
            min_amount: NearAmount::from_near(1),
            challenge_ttl: 300, // 5 minutes
            confirmations: 12,
            cache_ttl: 3600, // 1 hour
        }
    }
}

/// Cache entry
#[derive(Debug, Clone)]
struct CacheEntry {
    credential: NearCredential,
    verified_at: Instant,
}

/// NEAR payment verifier
pub struct NearVerifier {
    config: VerifierConfig,
    client: JsonRpcClient,
    pending_challenges: Arc<RwLock<HashMap<String, NearChallenge>>>,
    verified_cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
}

impl NearVerifier {
    /// Create new verifier
    pub fn new(config: VerifierConfig) -> Result<Self> {
        let client = JsonRpcClient::connect(&config.rpc_url);
        
        Ok(Self {
            config,
            client,
            pending_challenges: Arc::new(RwLock::new(HashMap::new())),
            verified_cache: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    /// Create a payment challenge
    pub async fn create_challenge(&self, amount: NearAmount) -> Result<NearChallenge> {
        let challenge_id = uuid::Uuid::new_v4().to_string();
        let nonce = uuid::Uuid::new_v4().to_string();
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        
        let challenge = NearChallenge {
            challenge_id: challenge_id.clone(),
            amount,
            recipient: self.config.recipient_account.clone(),
            method: "near".to_string(),
            expires_at: now + (self.config.challenge_ttl * 1_000_000_000), // nanoseconds
            nonce,
        };
        
        // Store pending challenge
        {
            let mut pending = self.pending_challenges.write().await;
            pending.insert(challenge_id, challenge.clone());
        }
        
        info!("Created challenge: {} (amount: {})", challenge.challenge_id, amount);
        Ok(challenge)
    }
    
    /// Verify a payment credential
    pub async fn verify(&self, credential: &NearCredential) -> Result<bool> {
        // Check cache
        {
            let cache = self.verified_cache.read().await;
            if let Some(entry) = cache.get(&credential.tx_hash.to_string()) {
                if entry.verified_at.elapsed().as_secs() < self.config.cache_ttl {
                    debug!("Using cached verification for {}", credential.tx_hash);
                    return Ok(true);
                }
            }
        }
        
        // Verify on-chain transaction (simplified - would need proper RPC call)
        // For now, just verify the credential structure
        if credential.recipient != self.config.recipient_account {
            return Err(Error::VerificationFailed(format!(
                "Wrong recipient: expected {}, got {}",
                self.config.recipient_account,
                credential.recipient
            )));
        }
        
        // Verify amount
        if credential.amount.0 < self.config.min_amount.0 {
            return Err(Error::VerificationFailed(format!(
                "Amount too low: minimum {}",
                self.config.min_amount
            )));
        }
        
        // Cache the verification
        {
            let mut cache = self.verified_cache.write().await;
            cache.insert(
                credential.tx_hash.to_string(),
                CacheEntry {
                    credential: credential.clone(),
                    verified_at: Instant::now(),
                },
            );
        }
        
        info!("Verified payment: {} from {}", credential.amount, credential.payer);
        Ok(true)
    }
    
    /// Get current block height
    async fn get_block_height(&self) -> Result<u64> {
        let request = near_jsonrpc_client::methods::block::RpcBlockRequest {
            block_reference: near_primitives::types::BlockReference::Finality(
                near_primitives::types::Finality::Final
            ),
        };
        
        let response = self.client.call(request).await.map_err(|e| {
            Error::RpcError(format!("Failed to get block: {:?}", e))
        })?;
        
        Ok(response.header.height)
    }
    
    /// Clean expired challenges
    pub async fn cleanup_expired(&self) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        
        let mut pending = self.pending_challenges.write().await;
        pending.retain(|_, challenge| challenge.expires_at > now);
    }
    
    /// Get pending challenges count
    pub async fn pending_count(&self) -> usize {
        self.pending_challenges.read().await.len()
    }
    
    /// Create a charge challenge
    pub async fn charge(&self, amount: &str) -> Result<NearChallenge> {
        let near_amount = NearAmount::from_near(amount.parse().map_err(|_| {
            Error::InvalidChallenge(format!("Invalid amount: {}", amount))
        })?);
        
        self.create_challenge(near_amount).await
    }
}
