//! Example: Gasless payments via NEAR Intents (OutLayer custody wallet)

use mpp_near::client::{IntentsProvider, IntentsConfig};
use mpp_near::types::{AccountId, NearAmount};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup logging
    tracing_subscriber::fmt::init();
    
    // Get wallet API key from environment
    let api_key = std::env::var("OUTLAYER_API_KEY")
        .expect("OUTLAYER_API_KEY not set. Get it from https://outlayer.fastnear.com");
    
    println!("Setting up NEAR Intents provider...");
    
    // Create provider
    let config = IntentsConfig {
        api_key,
        ..Default::default()
    };
    
    let provider = IntentsProvider::with_config(config);
    
    // Get account ID
    let account_id = provider.get_account_id().await?;
    println!("Wallet account: {}", account_id);
    
    // Check NEAR balance (for gas operations)
    let near_balance = provider.check_balance().await?;
    println!("NEAR balance: {}", near_balance);
    
    // Check intents balance (for gasless operations)
    let usdc_token = "17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1";
    let usdc_balance = provider.check_intents_balance(usdc_token).await?;
    println!("USDC intents balance: {}", usdc_balance);
    
    // List available tokens
    println!("\nFetching available tokens...");
    let tokens = provider.list_tokens().await?;
    println!("Found {} tokens", tokens.len());
    
    // Show first 5 tokens
    for token in tokens.iter().take(5) {
        println!("  {} ({}) - {}", token.symbol, token.chain, token.name);
    }
    
    // Example: Gasless transfer
    if near_balance.0 > 0 {
        println!("\nExample: Gasless transfer");
        let recipient = AccountId::new("receiver.near")?;
        let amount = NearAmount::from_near(1);
        
        // This would execute a gasless transfer
        println!("Would transfer {} to {} (gasless)", amount, recipient);
        // let tx_hash = provider.transfer(&recipient, amount).await?;
        // println!("Transaction: {}", tx_hash);
    }
    
    // Example: Payment check (agent-to-agent)
    if usdc_balance.0 > 0 {
        println!("\nExample: Payment check");
        let amount = NearAmount::from_usdc(10); // 10 USDC
        
        // This would create a payment check
        println!("Would create payment check for {}", amount);
        // let check = provider.create_payment_check(
        //     usdc_token,
        //     amount,
        //     Some("Payment for services"),
        //     Some(86400), // 24 hours
        // ).await?;
        // println!("Check ID: {}", check.check_id);
        // println!("Check key: {}", check.check_key);
    }
    
    // Example: Swap tokens
    println!("\nExample: Token swap");
    println!("Would swap wNEAR to USDC (gasless)");
    // let result = provider.swap(
    //     "nep141:wrap.near",
    //     "nep141:17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1",
    //     NearAmount::from_near(1),
    //     None,
    // ).await?;
    // println!("Swapped! Got {} USDC", result.amount_out);
    
    println!("\n✅ Intents provider ready for gasless operations!");
    
    Ok(())
}
