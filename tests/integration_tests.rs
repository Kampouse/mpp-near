//! Integration tests for mpp-near

#[cfg(test)]
mod tests {
    use mpp_near::types::{AccountId, Gas, NearAmount};
    
    #[test]
    fn test_account_id_validation() {
        // Valid account IDs
        assert!(AccountId::new("kampouse.near").is_ok());
        assert!(AccountId::new("invalid..near").is_err()); // Empty part
        assert!(AccountId::new("app.kampouse.near").is_ok());
        assert!(AccountId::new("a.near").is_ok()); // Single char is valid
        assert!(AccountId::new("valid.subdomain").is_ok());
        
        // Invalid cases
        assert!(AccountId::new("").is_err()); // Empty string
        assert!(AccountId::new(".near").is_err()); // Starts with dot
        assert!(AccountId::new("near.").is_err()); // Ends with dot
        assert!(AccountId::new("user@near").is_err()); // Invalid character
    }

    #[test]
    fn test_near_amount_from_near() {
        let amount = NearAmount::from_near(1);
        assert_eq!(amount.as_near(), 1);
    }

    #[test]
    fn test_near_amount_from_usdc() {
        let amount = NearAmount::from_usdc(100);
        assert_eq!(amount.0, 100_000_000);
    }

    #[test]
    fn test_gas_conversion() {
        let gas = Gas::tera(100);
        assert_eq!(gas.as_tgas(), 100);
    }

    #[cfg(feature = "client")]
    #[test]
    fn test_near_config_creation() {
        use mpp_near::client::NearConfig;
        use mpp_near::types::{AccountId, Gas, NearAmount};
        
        let config = NearConfig {
            rpc_url: "https://rpc.mainnet.near.org".to_string(),
            account_id: AccountId::new("kampouse.near").unwrap(),
            gas: Gas::DEFAULT,
            max_amount: NearAmount::from_near(10),
            network: "mainnet".to_string(),
            balance_cache_ttl: 30,
        };
        
        assert_eq!(config.network, "mainnet");
        assert_eq!(config.account_id.as_str(), "kampouse.near");
    }

    #[cfg(feature = "intents")]
    #[test]
    fn test_intents_config_creation() {
        use mpp_near::client::IntentsConfig;
        
        let config = IntentsConfig {
            api_key: "wk_test123".to_string(),
            api_url: "https://api.outlayer.fastnear.com".to_string(),
            default_chain: "near".to_string(),
            balance_cache_ttl: 30,
        };
        
        assert_eq!(config.api_key, "wk_test123");
        assert_eq!(config.api_url, "https://api.outlayer.fastnear.com");
    }

    #[cfg(feature = "server")]
    #[test]
    fn test_verifier_config() {
        use mpp_near::server::VerifierConfig;
        use mpp_near::types::{AccountId, NearAmount};
        
        let config = VerifierConfig {
            rpc_url: "https://rpc.mainnet.near.org".to_string(),
            recipient_account: AccountId::new("merchant.near").unwrap(),
            min_amount: NearAmount::from_near(1),
            challenge_ttl: 300,
            confirmations: 12,
            cache_ttl: 3600,
        };
        
        assert_eq!(config.recipient_account.as_str(), "merchant.near");
        assert_eq!(config.min_amount.as_near(), 1);
    }
}
