//! NEAR signer for creating signatures

use ed25519_dalek::{Signature, Signer as _, SigningKey, Verifier};
use near_crypto::{PublicKey, SecretKey};
use std::str::FromStr;

use crate::types::AccountId;
use crate::{Error, Result};

/// NEAR signer error
#[derive(Debug, thiserror::Error)]
pub enum SignerError {
    #[error("Invalid private key: {0}")]
    InvalidPrivateKey(String),
    
    #[error("Signature error: {0}")]
    SignatureError(String),
    
    #[error("Key derivation error: {0}")]
    KeyDerivationError(String),
}

/// NEAR signer
pub struct NearSigner {
    account_id: AccountId,
    secret_key: SecretKey,
    signing_key: SigningKey,
    public_key: PublicKey,
}

impl NearSigner {
    /// Create a new signer from private key
    pub fn new(account_id: AccountId, private_key: String) -> Result<Self> {
        // Parse NEAR private key format: "ed25519:base58encodedkey"
        let secret_key = SecretKey::from_str(&private_key)
            .map_err(|e| Error::InvalidSignature(format!("Invalid secret key: {:?}", e)))?;
        
        // Get the inner ED25519 secret key bytes (64 bytes: seed + public key)
        let key_bytes = match &secret_key {
            SecretKey::ED25519(k) => &k.0,
            _ => return Err(Error::InvalidSignature("Only ED25519 keys supported".to_string())),
        };
        
        // ed25519-dalek expects 32-byte seed (first 32 bytes of 64-byte key)
        let seed: [u8; 32] = key_bytes[..32]
            .try_into()
            .map_err(|_| Error::InvalidSignature("Invalid key length".to_string()))?;
        
        let signing_key = SigningKey::from_bytes(&seed);
        
        let public_key = secret_key.public_key();
        
        Ok(Self {
            account_id,
            secret_key,
            signing_key,
            public_key,
        })
    }
    
    /// Sign a challenge
    pub fn sign_challenge(&self, challenge_id: &str) -> Result<String> {
        let message = format!("{}:{}", self.account_id, challenge_id);
        let signature = self.signing_key.sign(message.as_bytes());
        
        Ok(format!("ed25519:{}", bs58::encode(signature.to_bytes()).into_string()))
    }
    
    /// Verify a signature
    pub fn verify(&self, message: &str, signature: &str) -> Result<bool> {
        // Parse signature
        let sig_str = signature.strip_prefix("ed25519:")
            .ok_or_else(|| Error::InvalidSignature("Missing ed25519 prefix".to_string()))?;
        
        let sig_bytes = bs58::decode(sig_str)
            .into_vec()
            .map_err(|e| Error::InvalidSignature(format!("Base58 decode error: {:?}", e)))?;
        
        let signature = Signature::from_bytes(
            &sig_bytes.try_into().map_err(|_| 
                Error::InvalidSignature("Invalid signature length".to_string())
            )?
        );
        
        // Verify
        let verifying_key = self.signing_key.verifying_key();
        match verifying_key.verify(message.as_bytes(), &signature) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Get public key
    pub fn public_key(&self) -> Result<PublicKey> {
        Ok(self.public_key.clone())
    }

    /// Get account ID
    pub fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    /// Sign arbitrary bytes
    pub fn sign_bytes(&self, bytes: &[u8]) -> Result<[u8; 64]> {
        let signature = self.signing_key.sign(bytes);
        Ok(signature.to_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::OsRng;
    
    #[test]
    fn test_sign_and_verify() {
        let account_id = AccountId::new("test.near").unwrap();
        
        // Generate a random key for testing
        let signing_key = SigningKey::generate(&mut OsRng);
        
        // Get verifying key and create full 64-byte NEAR secret key
        let verifying_key = signing_key.verifying_key();
        let mut near_key_bytes = [0u8; 64];
        near_key_bytes[..32].copy_from_slice(&signing_key.to_bytes());
        near_key_bytes[32..].copy_from_slice(&verifying_key.to_bytes());
        
        let near_secret = near_crypto::ED25519SecretKey(near_key_bytes);
        let secret_key = SecretKey::ED25519(near_secret);
        
        let key_str = format!("ed25519:{}", bs58::encode(&near_key_bytes).into_string());
        
        let signer = NearSigner::new(account_id, key_str).unwrap();
        
        // Test signing
        let signature = signer.sign_challenge("test-challenge").unwrap();
        assert!(signature.starts_with("ed25519:"));
        
        // Verify signature
        let message = format!("{}:{}", "test.near", "test-challenge");
        assert!(signer.verify(&message, &signature).unwrap());
    }
}
