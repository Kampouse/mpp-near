//! Bridge types and data structures

use serde::{Deserialize, Serialize};

/// Bridge payment request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeRequest {
    /// Challenge nonce from MPP server
    pub nonce: String,
    
    /// Recipient address on target chain
    pub recipient: String,
    
    /// Amount to pay (in USD)
    pub amount: f64,
    
    /// Token symbol (usdc, usdt, etc.)
    pub token: String,
    
    /// Target chain (ethereum, solana, bitcoin, etc.)
    pub target_chain: String,
    
    /// Optional: Challenge string for receipt binding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge: Option<String>,
}

/// Bridge payment response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeResponse {
    /// NEAR payment transaction hash
    pub near_tx: String,
    
    /// Target chain transaction hash (e.g., Ethereum tx hash)
    pub target_tx: String,
    
    /// Target chain name
    pub target_chain: String,
    
    /// Payment status
    pub status: BridgeStatus,
    
    /// Amount paid (confirmed)
    pub amount: f64,
    
    /// Token used
    pub token: String,
    
    /// Timestamp of completion
    pub timestamp: i64,
    
    /// Optional: Estimated confirmation time (for pending status)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_confirmation_ms: Option<u64>,
}

/// Bridge payment status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BridgeStatus {
    /// Payment received on NEAR, cross-chain pending
    Pending,
    
    /// Cross-chain transaction submitted, waiting for confirmation
    Submitted,
    
    /// Payment confirmed on target chain
    Confirmed,
    
    /// Payment failed
    Failed,
}

/// Bridge error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeError {
    /// Error type
    pub error: String,
    
    /// Human-readable message
    pub message: String,
    
    /// Optional error code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
}

impl std::fmt::Display for BridgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.error, self.message)
    }
}

impl std::error::Error for BridgeError {}

/// Supported chains for cross-chain payments
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Chain {
    Near,
    Ethereum,
    Solana,
    Bitcoin,
    Arbitrum,
    Base,
    Polygon,
    Optimism,
    Avalanche,
    Bsc,
    Ton,
    Aptos,
    Sui,
    Starknet,
    Tron,
    Stellar,
    Dogecoin,
    Xrp,
    Zcash,
    Litecoin,
}

impl Chain {
    /// Get chain name as string
    pub fn as_str(&self) -> &'static str {
        match self {
            Chain::Near => "near",
            Chain::Ethereum => "ethereum",
            Chain::Solana => "solana",
            Chain::Bitcoin => "bitcoin",
            Chain::Arbitrum => "arbitrum",
            Chain::Base => "base",
            Chain::Polygon => "polygon",
            Chain::Optimism => "optimism",
            Chain::Avalanche => "avalanche",
            Chain::Bsc => "bsc",
            Chain::Ton => "ton",
            Chain::Aptos => "aptos",
            Chain::Sui => "sui",
            Chain::Starknet => "starknet",
            Chain::Tron => "tron",
            Chain::Stellar => "stellar",
            Chain::Dogecoin => "dogecoin",
            Chain::Xrp => "xrp",
            Chain::Zcash => "zcash",
            Chain::Litecoin => "litecoin",
        }
    }
    
    /// Parse from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "near" => Some(Chain::Near),
            "ethereum" | "eth" => Some(Chain::Ethereum),
            "solana" | "sol" => Some(Chain::Solana),
            "bitcoin" | "btc" => Some(Chain::Bitcoin),
            "arbitrum" | "arb" => Some(Chain::Arbitrum),
            "base" => Some(Chain::Base),
            "polygon" | "matic" => Some(Chain::Polygon),
            "optimism" | "op" => Some(Chain::Optimism),
            "avalanche" | "avax" => Some(Chain::Avalanche),
            "bsc" | "binance" => Some(Chain::Bsc),
            "ton" => Some(Chain::Ton),
            "aptos" | "apt" => Some(Chain::Aptos),
            "sui" => Some(Chain::Sui),
            "starknet" => Some(Chain::Starknet),
            "tron" => Some(Chain::Tron),
            "stellar" | "xlm" => Some(Chain::Stellar),
            "dogecoin" | "doge" => Some(Chain::Dogecoin),
            "xrp" => Some(Chain::Xrp),
            "zcash" | "zec" => Some(Chain::Zcash),
            "litecoin" | "ltc" => Some(Chain::Litecoin),
            _ => None,
        }
    }
    
    /// Check if address format is valid for this chain
    pub fn is_valid_address(&self, address: &str) -> bool {
        match self {
            Chain::Ethereum | Chain::Arbitrum | Chain::Base | Chain::Polygon | 
            Chain::Optimism | Chain::Avalanche | Chain::Bsc => {
                address.starts_with("0x") && address.len() == 42
            }
            Chain::Bitcoin => {
                address.starts_with("1") || address.starts_with("3") || 
                address.starts_with("bc1")
            }
            Chain::Solana => {
                // Base58 encoded, typically 32-44 chars
                address.len() >= 32 && address.len() <= 44
            }
            Chain::Near => {
                address.ends_with(".near") || address.ends_with(".testnet") ||
                address.len() == 64 && address.chars().all(|c| c.is_ascii_hexdigit())
            }
            _ => !address.is_empty() // Generic check for other chains
        }
    }
}

/// Token info for cross-chain payments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    /// Token symbol
    pub symbol: String,
    
    /// Token ID on NEAR
    pub near_token_id: String,
    
    /// Decimals
    pub decimals: u8,
}

impl TokenInfo {
    /// Get token info by symbol
    pub fn from_symbol(symbol: &str) -> Option<Self> {
        match symbol.to_lowercase().as_str() {
            "usdc" => Some(TokenInfo {
                symbol: "USDC".into(),
                near_token_id: "17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1".into(),
                decimals: 6,
            }),
            "usdt" => Some(TokenInfo {
                symbol: "USDT".into(),
                near_token_id: "usdt.tether-token.near".into(),
                decimals: 6,
            }),
            "btc" => Some(TokenInfo {
                symbol: "BTC".into(),
                near_token_id: "btc.omft.near".into(),
                decimals: 8,
            }),
            "eth" => Some(TokenInfo {
                symbol: "ETH".into(),
                near_token_id: "eth.omft.near".into(),
                decimals: 18,
            }),
            "sol" => Some(TokenInfo {
                symbol: "SOL".into(),
                near_token_id: "sol.omft.near".into(),
                decimals: 9,
            }),
            _ => None,
        }
    }
}
