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
        use near_primitives::{
            transaction::{Action, TransferAction},
            types::{BlockReference},
            views::{AccessKeyView, QueryRequest},
        };
        use near_jsonrpc_client::methods::{
            block::RpcBlockRequest,
            query::RpcQueryRequest,
            broadcast_tx_commit::RpcBroadcastTxCommitRequest,
        };

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

        // Get latest block hash
        let block_request = RpcBlockRequest {
            block_reference: BlockReference::latest(),
        };
        let block_response = self.client.call(block_request).await
            .map_err(|e| Error::TransactionFailed(format!("Failed to get block: {}", e)))?;
        let block_hash = block_response.header.hash;

        // Get public key
        let public_key = self.signer.public_key()?;

        // Get the actual nonce by querying the ACCESS KEY (not the account!)
        // Each access key has its own nonce on NEAR
        let access_key_query = RpcQueryRequest {
            request: QueryRequest::ViewAccessKey {
                account_id: self.config.account_id.as_str().parse()
                    .map_err(|e| Error::InvalidAccountId(format!("Invalid account ID: {}", e)))?,
                public_key: public_key.clone(),
            },
            block_reference: BlockReference::latest(),
        };

        let access_key_nonce = match self.client.call(access_key_query).await {
            Ok(response) => {
                let json = serde_json::to_string(&response).unwrap_or_default();
                debug!("Access key response: {}", json);
                if let Ok(value) = serde_json::from_str::<serde_json::Value>(&json) {
                    // The response structure has nonce directly at root level
                    let nonce = value.get("nonce")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(0u64);
                    // Increment nonce because each transaction attempt uses it
                    nonce + 1
                } else {
                    0u64
                }
            }
            Err(e) => {
                return Err(Error::TransactionFailed(format!(
                    "Access key for '{}' does not exist on the network. \
                    Please make sure the public key derived from your private key exists on this account.\n\
                    You can check access keys with: ./target/debug/check_keys {}\nError: {}",
                    self.config.account_id, self.config.account_id, e
                )));
            }
        };

        let nonce = access_key_nonce;

        debug!("Using nonce: {}", nonce);

        // Create and sign transaction
        let signed_tx = self.create_signed_transaction(
            recipient,
            amount,
            &block_hash,
            nonce,
        )?;

        let sig_hex = match &signed_tx.signature {
            near_crypto::Signature::ED25519(sig) => hex::encode(sig.to_bytes()),
            _ => "unknown".to_string(),
        };

        debug!("Signed transaction: signer={}, receiver={}, nonce={}, public_key={}, signature={}",
            signed_tx.transaction.signer_id(),
            signed_tx.transaction.receiver_id(),
            signed_tx.transaction.nonce(),
            signed_tx.transaction.public_key(),
            sig_hex
        );

        // Broadcast transaction
        let broadcast_request = RpcBroadcastTxCommitRequest {
            signed_transaction: signed_tx.clone(),
        };

        let response = self.client.call(broadcast_request).await
            .map_err(|e| Error::TransactionFailed(format!("Failed to broadcast transaction: {}", e)))?;

        // Get transaction hash from the outcome
        let tx_hash = format!("0x{}", hex::encode(response.transaction_outcome.id.as_ref()));

        info!("Transaction sent with hash: {}", tx_hash);
        TransactionHash::new(tx_hash)
    }

    /// Create and sign a transaction
    fn create_signed_transaction(
        &self,
        recipient: &AccountId,
        amount: NearAmount,
        block_hash: &near_primitives::hash::CryptoHash,
        nonce: u64,
    ) -> Result<near_primitives::transaction::SignedTransaction> {
        use near_primitives::{
            transaction::{Action, TransferAction},
            borsh::BorshSerialize,
        };

        // Parse account IDs
        let signer_id: near_primitives::types::AccountId = self.config.account_id.as_str().parse()
            .map_err(|e| Error::InvalidAccountId(format!("Invalid signer ID: {}", e)))?;
        let receiver_id: near_primitives::types::AccountId = recipient.as_str().parse()
            .map_err(|e| Error::InvalidAccountId(format!("Invalid receiver ID: {}", e)))?;

        // Get public key
        let public_key = self.signer.public_key()?;
        debug!("Creating transaction with public key: {}", public_key);

        // Create transfer action
        let action = Action::Transfer(TransferAction { deposit: amount.0 });
        debug!("Created action: Transfer with deposit: {}", amount.0);

        // Create transaction (TransactionV0)
        let transaction_v0 = near_primitives::transaction::TransactionV0 {
            signer_id: signer_id.clone(),
            public_key: public_key.clone(),
            nonce,
            receiver_id,
            block_hash: *block_hash,
            actions: vec![action],
        };

        // Wrap in Transaction enum
        let transaction = near_primitives::transaction::Transaction::V0(transaction_v0);

        // Sign transaction
        let signature = self.sign_transaction(&transaction)?;

        Ok(near_primitives::transaction::SignedTransaction::new(
            signature,
            transaction,
        ))
    }

    /// Sign a transaction
    fn sign_transaction(
        &self,
        transaction: &near_primitives::transaction::Transaction,
    ) -> Result<near_crypto::Signature> {
        use near_primitives::borsh::BorshSerialize;
        use near_primitives::hash::hash;

        // Serialize transaction
        let mut buffer = Vec::new();
        transaction.serialize(&mut buffer)
            .map_err(|e| Error::InvalidSignature(format!("Failed to serialize transaction: {}", e)))?;

        debug!("Signing transaction ({} bytes)", buffer.len());
        debug!("Transaction bytes (hex): {}", hex::encode(&buffer));

        // Sign the hash of the transaction
        let tx_hash = hash(&buffer);
        debug!("Transaction hash: {}", hex::encode(tx_hash.as_ref()));

        let sig_bytes = self.signer.sign_bytes(tx_hash.as_ref())?;

        debug!("Signature: {}", hex::encode(&sig_bytes));

        // Create ed25519 signature from bytes and wrap in near_crypto::Signature
        let ed_sig = ed25519_dalek::Signature::from_bytes(&sig_bytes);

        Ok(near_crypto::Signature::ED25519(ed_sig))
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
        use near_jsonrpc_client::methods::block::RpcBlockRequest;

        let request = RpcBlockRequest {
            block_reference: near_primitives::types::BlockReference::latest(),
        };

        let response = self.client.call(request).await
            .map_err(|e| Error::TransactionFailed(format!("Failed to get block height: {}", e)))?;

        Ok(response.header.height)
    }
    
    /// Get account ID
    pub fn account_id(&self) -> &AccountId {
        &self.config.account_id
    }
}
