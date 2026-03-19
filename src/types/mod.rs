//! NEAR-specific types for MPP

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// NEAR account ID (e.g., "kampouse.near" or "64-character hex")
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AccountId(String);

impl AccountId {
    pub fn new(account_id: impl Into<String>) -> Result<Self, super::Error> {
        let id = account_id.into();
        
        // Validate NEAR account ID rules
        let parts: Vec<&str> = id.split('.').collect();
        for part in &parts {
            if part.is_empty() || part.len() > 63 {
                return Err(super::Error::InvalidAccountId(id));
            }
            if !part.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
                return Err(super::Error::InvalidAccountId(id));
            }
        }
        
        Ok(Self(id))
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for AccountId {
    type Err = super::Error;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl fmt::Display for AccountId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Gas units (Tgas = 10^12 gas)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Gas(pub u64);

impl Gas {
    /// 1 Tera gas (10^12)
    pub const TERA: u64 = 1_000_000_000_000;
    
    /// Default gas for function calls (100 Tgas)
    pub const DEFAULT: Self = Gas(100 * Self::TERA);
    
    /// Maximum gas per transaction (300 Tgas)
    pub const MAX: Self = Gas(300 * Self::TERA);
    
    pub fn tera(tgas: u64) -> Self {
        Self(tgas * Self::TERA)
    }
    
    pub fn as_tgas(&self) -> u64 {
        self.0 / Self::TERA
    }
}

/// NEAR amount in yoctoNEAR (10^-24 NEAR)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct NearAmount(pub u128);

impl NearAmount {
    /// 1 NEAR in yoctoNEAR
    pub const NEAR: u128 = 1_000_000_000_000_000_000_000_000; // 10^24
    
    /// Create from NEAR (converts to yoctoNEAR)
    pub fn from_near(near: u64) -> Self {
        Self(near as u128 * Self::NEAR)
    }
    
    /// Create from yoctoNEAR
    pub fn from_yocto(yocto: u128) -> Self {
        Self(yocto)
    }
    
    /// Create from USDC (6 decimals)
    pub fn from_usdc(usdc: u64) -> Self {
        Self(usdc as u128 * 1_000_000) // USDC has 6 decimals
    }
    
    /// Convert to NEAR (lossy)
    pub fn as_near(&self) -> u64 {
        (self.0 / Self::NEAR) as u64
    }
    
    /// Format as human-readable NEAR
    pub fn format_near(&self) -> String {
        let near = self.0 as f64 / Self::NEAR as f64;
        format!("{:.6} NEAR", near)
    }
}

impl fmt::Display for NearAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_near())
    }
}

/// Transaction hash (64-character hex)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TransactionHash(String);

impl TransactionHash {
    pub fn new(hash: impl Into<String>) -> Result<Self, super::Error> {
        let h = hash.into();
        if h.len() != 44 || !h.starts_with("0x") {
            return Err(super::Error::InvalidSignature(format!("Invalid tx hash: {}", h)));
        }
        Ok(Self(h))
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for TransactionHash {
    type Err = super::Error;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl fmt::Display for TransactionHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Payment credential returned after successful NEAR payment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearCredential {
    /// Transaction hash
    pub tx_hash: TransactionHash,
    /// Payer account ID
    pub payer: AccountId,
    /// Recipient account ID
    pub recipient: AccountId,
    /// Amount paid (yoctoNEAR)
    pub amount: NearAmount,
    /// Block height of transaction
    pub block_height: u64,
    /// Signature over challenge
    pub signature: String,
    /// Timestamp (Unix nanoseconds)
    pub timestamp: u64,
}

/// Challenge sent by server in 402 response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearChallenge {
    /// Unique challenge ID
    pub challenge_id: String,
    /// Amount to pay (yoctoNEAR)
    pub amount: NearAmount,
    /// Recipient account ID
    pub recipient: AccountId,
    /// Payment method (e.g., "near", "usdc")
    pub method: String,
    /// Expiration timestamp (Unix nanoseconds)
    pub expires_at: u64,
    /// Nonce for replay protection
    pub nonce: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_account_id_validation() {
        assert!(AccountId::new("kampouse.near").is_ok());
        assert!(AccountId::new("app.kampouse.near").is_ok());
        assert!(AccountId::new("invalid..near").is_err());
        assert!(AccountId::new("").is_err());
    }
    
    #[test]
    fn test_near_amount() {
        let amount = NearAmount::from_near(1);
        assert_eq!(amount.as_near(), 1);
        assert_eq!(NearAmount::from_usdc(100).0, 100_000_000);
    }
    
    #[test]
    fn test_gas() {
        assert_eq!(Gas::tera(100).as_tgas(), 100);
        assert_eq!(Gas::DEFAULT.as_tgas(), 100);
    }
}
