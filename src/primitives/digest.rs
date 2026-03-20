//! MPP Body Digest - Request body binding per RFC 9530
//!
//! Body digests bind challenges to specific request bodies, preventing
//! clients from modifying the body after receiving a challenge.

use sha2::{Sha256, Sha512, Digest};
use base64::Engine;

/// Supported digest algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DigestAlgorithm {
    /// SHA-256
    Sha256,
    /// SHA-512
    Sha512,
}

impl DigestAlgorithm {
    /// Parse from string (e.g., "sha-256", "sha-512")
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "sha-256" | "sha256" => Some(Self::Sha256),
            "sha-512" | "sha512" => Some(Self::Sha512),
            _ => None,
        }
    }
    
    /// Get the algorithm name
    pub fn name(&self) -> &'static str {
        match self {
            DigestAlgorithm::Sha256 => "sha-256",
            DigestAlgorithm::Sha512 => "sha-512",
        }
    }
}

/// Body digest wrapper
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BodyDigest {
    /// Algorithm used
    pub algorithm: DigestAlgorithm,
    /// Base64-encoded hash
    pub hash: String,
}

impl BodyDigest {
    /// Create a new body digest
    pub fn new(algorithm: DigestAlgorithm, hash: impl Into<String>) -> Self {
        Self {
            algorithm,
            hash: hash.into(),
        }
    }
    
    /// Compute digest for a body using SHA-256
    pub fn sha256(body: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(body);
        let hash = base64::engine::general_purpose::STANDARD.encode(hasher.finalize());
        Self {
            algorithm: DigestAlgorithm::Sha256,
            hash,
        }
    }
    
    /// Compute digest for a body using SHA-512
    pub fn sha512(body: &[u8]) -> Self {
        let mut hasher = Sha512::new();
        hasher.update(body);
        let hash = base64::engine::general_purpose::STANDARD.encode(hasher.finalize());
        Self {
            algorithm: DigestAlgorithm::Sha512,
            hash,
        }
    }
    
    /// Parse from Content-Digest header format
    ///
    /// Format: `algorithm=:base64hash:`
    /// Example: `sha-256=:X48E9qOokqqrvdts8nOJRJN3OWDUoyWxBf7kbu9DBPE=:`
    pub fn from_header(header: &str) -> Option<Self> {
        // Format: algorithm=:hash:
        // Find the first "=:" to split algorithm and hash
        let idx = header.find("=:")?;
        let algorithm = DigestAlgorithm::from_str(&header[..idx])?;
        
        // Extract hash between "=:" and final ":"
        let hash_part = &header[idx + 2..];
        let hash = hash_part.strip_suffix(':')?.to_string();
        
        Some(Self { algorithm, hash })
    }
    
    /// Convert to Content-Digest header format
    pub fn to_header(&self) -> String {
        format!("{}=:{}:", self.algorithm.name(), self.hash)
    }
    
    /// Verify that a body matches this digest
    pub fn verify(&self, body: &[u8]) -> bool {
        let expected = match self.algorithm {
            DigestAlgorithm::Sha256 => Self::sha256(body),
            DigestAlgorithm::Sha512 => Self::sha512(body),
        };
        expected.hash == self.hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sha256_digest() {
        let body = b"hello world";
        let digest = BodyDigest::sha256(body);
        
        assert_eq!(digest.algorithm, DigestAlgorithm::Sha256);
        assert!(!digest.hash.is_empty());
    }
    
    #[test]
    fn test_header_roundtrip() {
        let body = b"test body content";
        let original = BodyDigest::sha256(body);
        let header = original.to_header();
        let parsed = BodyDigest::from_header(&header).unwrap();
        
        assert_eq!(parsed.algorithm, original.algorithm);
        assert_eq!(parsed.hash, original.hash);
    }
    
    #[test]
    fn test_verify() {
        let body = b"test body";
        let digest = BodyDigest::sha256(body);
        
        assert!(digest.verify(body));
        assert!(!digest.verify(b"different body"));
    }
    
    #[test]
    fn test_from_header() {
        let header = "sha-256=:X48E9qOokqqrvdts8nOJRJN3OWDUoyWxBf7kbu9DBPE=:";
        let digest = BodyDigest::from_header(header).unwrap();
        
        assert_eq!(digest.algorithm, DigestAlgorithm::Sha256);
        assert_eq!(digest.hash, "X48E9qOokqqrvdts8nOJRJN3OWDUoyWxBf7kbu9DBPE=");
    }
}
