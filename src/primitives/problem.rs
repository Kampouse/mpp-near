//! MPP Problem - Error details following RFC 9457
//!
//! Problems are returned with 402 responses to provide detailed error information.

use serde::{Deserialize, Serialize};

/// Problem type URI prefix
pub const PROBLEM_TYPE_PREFIX: &str = "https://paymentauth.org/problems";

/// Problem types defined by MPP spec
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProblemType {
    /// Resource requires payment
    PaymentRequired,
    /// Amount too low
    PaymentInsufficient,
    /// Challenge or authorization expired
    PaymentExpired,
    /// Payment proof invalid
    VerificationFailed,
    /// Payment method not accepted
    MethodUnsupported,
    /// Invalid credential format
    MalformedCredential,
    /// Challenge ID unknown, expired, or already used
    InvalidChallenge,
    /// Custom problem type
    #[serde(untagged)]
    Custom(String),
}

impl ProblemType {
    /// Get the full URI for this problem type
    pub fn uri(&self) -> String {
        match self {
            ProblemType::PaymentRequired => format!("{}/payment-required", PROBLEM_TYPE_PREFIX),
            ProblemType::PaymentInsufficient => format!("{}/payment-insufficient", PROBLEM_TYPE_PREFIX),
            ProblemType::PaymentExpired => format!("{}/payment-expired", PROBLEM_TYPE_PREFIX),
            ProblemType::VerificationFailed => format!("{}/verification-failed", PROBLEM_TYPE_PREFIX),
            ProblemType::MethodUnsupported => format!("{}/method-unsupported", PROBLEM_TYPE_PREFIX),
            ProblemType::MalformedCredential => format!("{}/malformed-credential", PROBLEM_TYPE_PREFIX),
            ProblemType::InvalidChallenge => format!("{}/invalid-challenge", PROBLEM_TYPE_PREFIX),
            ProblemType::Custom(uri) => uri.clone(),
        }
    }
    
    /// Get the title for this problem type
    pub fn title(&self) -> &'static str {
        match self {
            ProblemType::PaymentRequired => "Payment Required",
            ProblemType::PaymentInsufficient => "Payment Insufficient",
            ProblemType::PaymentExpired => "Payment Expired",
            ProblemType::VerificationFailed => "Verification Failed",
            ProblemType::MethodUnsupported => "Method Unsupported",
            ProblemType::MalformedCredential => "Malformed Credential",
            ProblemType::InvalidChallenge => "Invalid Challenge",
            ProblemType::Custom(_) => "Unknown Error",
        }
    }
}

/// Problem Details object (RFC 9457)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Problem {
    /// URI reference identifying the problem type
    #[serde(rename = "type")]
    pub problem_type: String,
    
    /// Short, human-readable title
    pub title: String,
    
    /// HTTP status code
    pub status: u16,
    
    /// Detailed human-readable explanation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    
    /// URI for more information
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "help")]
    pub instance: Option<String>,
    
    /// Additional problem-specific fields
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl Problem {
    /// Create a new problem
    pub fn new(problem_type: ProblemType) -> Self {
        Self {
            problem_type: problem_type.uri(),
            title: problem_type.title().to_string(),
            status: 402,
            detail: None,
            instance: None,
            extra: std::collections::HashMap::new(),
        }
    }
    
    /// Create a payment-required problem
    pub fn payment_required() -> Self {
        Self::new(ProblemType::PaymentRequired)
    }
    
    /// Create a payment-insufficient problem
    pub fn payment_insufficient(required: &str, provided: &str) -> Self {
        let mut problem = Self::new(ProblemType::PaymentInsufficient);
        problem.detail = Some(format!("Required: {}, Provided: {}", required, provided));
        problem
    }
    
    /// Create a payment-expired problem
    pub fn payment_expired() -> Self {
        Self::new(ProblemType::PaymentExpired)
    }
    
    /// Create a verification-failed problem
    pub fn verification_failed(reason: &str) -> Self {
        let mut problem = Self::new(ProblemType::VerificationFailed);
        problem.detail = Some(reason.to_string());
        problem
    }
    
    /// Create a method-unsupported problem
    pub fn method_unsupported(method: &str) -> Self {
        let mut problem = Self::new(ProblemType::MethodUnsupported);
        problem.detail = Some(format!("Method '{}' not supported", method));
        problem
    }
    
    /// Create a malformed-credential problem
    pub fn malformed_credential(reason: &str) -> Self {
        let mut problem = Self::new(ProblemType::MalformedCredential);
        problem.detail = Some(reason.to_string());
        problem
    }
    
    /// Create an invalid-challenge problem
    pub fn invalid_challenge(reason: &str) -> Self {
        let mut problem = Self::new(ProblemType::InvalidChallenge);
        problem.detail = Some(reason.to_string());
        problem
    }
    
    /// Add detail
    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }
    
    /// Add instance URI
    pub fn with_instance(mut self, instance: impl Into<String>) -> Self {
        self.instance = Some(instance.into());
        self
    }
    
    /// Add extra field
    pub fn with_extra(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.extra.insert(key.into(), value);
        self
    }
    
    /// Set HTTP status code
    pub fn with_status(mut self, status: u16) -> Self {
        self.status = status;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_problem_types() {
        assert_eq!(
            ProblemType::PaymentRequired.uri(),
            "https://paymentauth.org/problems/payment-required"
        );
        assert_eq!(
            ProblemType::VerificationFailed.uri(),
            "https://paymentauth.org/problems/verification-failed"
        );
    }
    
    #[test]
    fn test_problem_serialization() {
        let problem = Problem::verification_failed("Invalid signature");
        let json = serde_json::to_string(&problem).unwrap();
        
        assert!(json.contains("verification-failed"));
        assert!(json.contains("Invalid signature"));
    }
    
    #[test]
    fn test_problem_builders() {
        let problem = Problem::payment_insufficient("0.01", "0.001")
            .with_status(402);
        
        assert_eq!(problem.status, 402);
        assert!(problem.detail.unwrap().contains("0.01"));
    }
}
