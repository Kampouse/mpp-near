//! MPP Receipt - Server acknowledgment of successful payment
//!
//! A Receipt is returned with HTTP 200 responses to confirm payment.

use serde::{Deserialize, Serialize};
use rand::Rng;
use chrono::Utc;

use crate::VERSION;

/// Payment receipt returned with 200 responses
///
/// A Receipt confirms that payment was successfully verified. It includes
/// details about the payment for the client's records.
///
/// # Example
///
/// ```
/// use mpp_near::Receipt;
///
/// let receipt = Receipt::builder()
///     .challenge_id("abc123")
///     .account("user.near")
///     .amount("0.001")
///     .token("USDC")
///     .build()
///     .unwrap();
///
/// // Serialize to Payment-Receipt header
/// let header = receipt.to_header();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Receipt {
    /// Unique receipt identifier
    #[serde(rename = "id")]
    pub id: String,
    
    /// MPP protocol version
    #[serde(rename = "version")]
    pub version: String,
    
    /// Challenge ID this receipt is for
    #[serde(rename = "challengeId")]
    pub challenge_id: String,
    
    /// Account that paid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    
    /// Amount paid
    #[serde(rename = "amount")]
    pub amount: String,
    
    /// Token paid
    #[serde(rename = "token")]
    pub token: String,
    
    /// Payment method used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    
    /// Unix timestamp when payment was confirmed
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
    
    /// Payment status
    #[serde(rename = "status")]
    pub status: String,
    
    /// Transaction/proof hash (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<String>,
    
    /// Additional metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl Receipt {
    /// Create a new receipt builder
    pub fn builder() -> ReceiptBuilder {
        ReceiptBuilder::new()
    }
    
    /// Generate a random receipt ID
    pub fn generate_id() -> String {
        hex::encode(rand::thread_rng().gen::<[u8; 16]>())
    }
    
    /// Create a receipt for a challenge and credential
    pub fn for_payment(challenge_id: &str, account: Option<&str>, amount: &str, token: &str) -> Self {
        Self {
            id: Self::generate_id(),
            version: VERSION.to_string(),
            challenge_id: challenge_id.to_string(),
            account: account.map(|s| s.to_string()),
            amount: amount.to_string(),
            token: token.to_string(),
            method: None,
            timestamp: Utc::now().timestamp(),
            status: "confirmed".to_string(),
            proof: None,
            metadata: None,
        }
    }
    
    /// Serialize to Payment-Receipt header value
    pub fn to_header(&self) -> String {
        let mut params = vec![
            format!(r#"id="{}""#, self.id),
            format!(r#"challengeId="{}""#, self.challenge_id),
            format!(r#"amount="{}""#, self.amount),
            format!(r#"token="{}""#, self.token),
            format!(r#"timestamp="{}""#, self.timestamp),
            format!(r#"status="{}""#, self.status),
        ];
        
        if let Some(ref account) = self.account {
            params.push(format!(r#"account="{}""#, account));
        }
        
        if let Some(ref method) = self.method {
            params.push(format!(r#"method="{}""#, method));
        }
        
        if let Some(ref proof) = self.proof {
            params.push(format!(r#"proof="{}""#, proof));
        }
        
        format!("Payment {}", params.join(", "))
    }
    
    /// Parse from Payment-Receipt header
    pub fn from_header(header: &str) -> Option<Self> {
        if !header.starts_with("Payment ") {
            return None;
        }
        
        let params = &header[8..];
        let mut builder = Receipt::builder();
        
        for part in params.split(',') {
            let part = part.trim();
            if let Some((key, value)) = part.split_once('=') {
                let value = value.trim_matches('"');
                match key.trim() {
                    "id" => builder = builder.id(value),
                    "challengeId" => builder = builder.challenge_id(value),
                    "account" => builder = builder.account(value),
                    "amount" => builder = builder.amount(value),
                    "token" => builder = builder.token(value),
                    "method" => builder = builder.method(value),
                    "timestamp" => {
                        if let Ok(ts) = value.parse() {
                            builder = builder.timestamp(ts);
                        }
                    }
                    "status" => builder = builder.status(value),
                    "proof" => builder = builder.proof(value),
                    _ => {}
                }
            }
        }
        
        builder.build().ok()
    }
}

/// Builder for creating receipts
#[derive(Debug, Default)]
pub struct ReceiptBuilder {
    id: Option<String>,
    challenge_id: Option<String>,
    account: Option<String>,
    amount: Option<String>,
    token: Option<String>,
    method: Option<String>,
    timestamp: Option<i64>,
    status: Option<String>,
    proof: Option<String>,
    metadata: Option<serde_json::Value>,
}

impl ReceiptBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set receipt ID (auto-generated if not set)
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
    
    /// Set challenge ID (required)
    pub fn challenge_id(mut self, challenge_id: impl Into<String>) -> Self {
        self.challenge_id = Some(challenge_id.into());
        self
    }
    
    /// Set account
    pub fn account(mut self, account: impl Into<String>) -> Self {
        self.account = Some(account.into());
        self
    }
    
    /// Set amount (required)
    pub fn amount(mut self, amount: impl Into<String>) -> Self {
        self.amount = Some(amount.into());
        self
    }
    
    /// Set token (required)
    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }
    
    /// Set method
    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }
    
    /// Set timestamp (auto-generated if not set)
    pub fn timestamp(mut self, timestamp: i64) -> Self {
        self.timestamp = Some(timestamp);
        self
    }
    
    /// Set status (default: "confirmed")
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }
    
    /// Set proof/transaction hash
    pub fn proof(mut self, proof: impl Into<String>) -> Self {
        self.proof = Some(proof.into());
        self
    }
    
    /// Set metadata
    pub fn metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
    
    /// Build the receipt
    pub fn build(self) -> Result<Receipt, &'static str> {
        let challenge_id = self.challenge_id.ok_or("challenge_id is required")?;
        let amount = self.amount.ok_or("amount is required")?;
        let token = self.token.ok_or("token is required")?;
        
        Ok(Receipt {
            id: self.id.unwrap_or_else(Receipt::generate_id),
            version: VERSION.to_string(),
            challenge_id,
            account: self.account,
            amount,
            token,
            method: self.method,
            timestamp: self.timestamp.unwrap_or_else(|| Utc::now().timestamp()),
            status: self.status.unwrap_or_else(|| "confirmed".to_string()),
            proof: self.proof,
            metadata: self.metadata,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_receipt_builder() {
        let receipt = Receipt::builder()
            .challenge_id("abc123")
            .account("user.near")
            .amount("0.001")
            .token("USDC")
            .build()
            .unwrap();
        
        assert_eq!(receipt.challenge_id, "abc123");
        assert_eq!(receipt.amount, "0.001");
        assert_eq!(receipt.status, "confirmed");
    }
    
    #[test]
    fn test_header_roundtrip() {
        let original = Receipt::builder()
            .id("receipt-123")
            .challenge_id("challenge-456")
            .account("user.near")
            .amount("0.001")
            .token("USDC")
            .timestamp(1000000000)
            .status("confirmed")
            .build()
            .unwrap();
        
        let header = original.to_header();
        let parsed = Receipt::from_header(&header).unwrap();
        
        assert_eq!(parsed.id, original.id);
        assert_eq!(parsed.challenge_id, original.challenge_id);
        assert_eq!(parsed.amount, original.amount);
    }
    
    #[test]
    fn test_for_payment() {
        let receipt = Receipt::for_payment("abc", Some("user"), "1.0", "NEAR");
        assert_eq!(receipt.challenge_id, "abc");
        assert_eq!(receipt.amount, "1.0");
        assert_eq!(receipt.token, "NEAR");
    }
}
