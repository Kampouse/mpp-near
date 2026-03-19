//! Example: NEAR client making paid API requests

use mpp_near::client::{NearConfig, NearProvider, PaymentMiddleware};
use mpp_near::types::{AccountId, NearAmount};
use reqwest_middleware::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup logging
    tracing_subscriber::fmt::init();
    
    // Configure NEAR provider
    let account_id: AccountId = std::env::var("NEAR_ACCOUNT_ID")
        .expect("NEAR_ACCOUNT_ID not set")
        .parse()?;
    
    let private_key = std::env::var("NEAR_PRIVATE_KEY")
        .expect("NEAR_PRIVATE_KEY not set");
    
    let rpc_url = std::env::var("NEAR_RPC_URL")
        .unwrap_or_else(|_| "https://rpc.mainnet.near.org".to_string());
    
    let config = NearConfig {
        rpc_url,
        account_id: account_id.clone(),
        max_amount: NearAmount::from_near(10), // Max 10 NEAR per payment
        ..Default::default()
    };
    
    println!("Setting up NEAR payment provider for: {}", account_id);
    
    // Create provider
    let provider = NearProvider::with_config(config, private_key)?;
    
    // Check balance
    let balance = provider.check_balance().await?;
    println!("Account balance: {}", balance);
    
    // Create HTTP client with payment middleware
    let client = ClientBuilder::new(reqwest::Client::new())
        .with(PaymentMiddleware::new(provider))
        .build();
    
    // Make paid API request
    println!("\nMaking paid API request...");
    let response = client
        .get("https://api.example.com/paid-endpoint")
        .send()
        .await?;
    
    println!("Response status: {}", response.status());
    println!("Response body: {}", response.text().await?);
    
    Ok(())
}
