//! Cross-chain MPP client example
//!
//! Demonstrates how to use the AgentClient to pay MPP servers on different chains
//! (Ethereum, Solana, Bitcoin, etc.) using NEAR Intents as the payment bridge.
//!
//! # How it works
//!
//! 1. Client sends request to MPP server on Ethereum
//! 2. Server returns 402: "Pay 1 USDC to 0xABC..."
//! 3. AgentClient detects Ethereum address
//! 4. AgentClient calls OutLayer cross-chain withdraw (NEAR → Ethereum)
//! 5. AgentClient gets Ethereum tx hash
//! 6. AgentClient submits credential with Ethereum tx hash
//! 7. Server verifies tx on Ethereum and returns data
//!
//! # Usage
//!
//! ```bash
//! export OUTLAYER_API_KEY="wk_your_key"
//! cargo run --example cross_chain_client
//! ```

use mpp_near::client::{AgentClient, BudgetConfig};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment
    let api_key = env::var("OUTLAYER_API_KEY")
        .expect("OUTLAYER_API_KEY environment variable required");

    // Create client with budget limits
    let client = AgentClient::new(&api_key)
        .with_budget(BudgetConfig::new(1.0, 10.0)); // Max $1 per request, $10 per day

    println!("🌉 Cross-Chain MPP Client Example");
    println!("{}\n", "━".repeat(50));

    // Example 1: Call Ethereum MPP server
    println!("Example 1: Ethereum MPP Server");
    println!("Calling: https://api.eth-example.com/data");
    println!("Expected: 402 → Pay to 0x... address → Auto-bridge → Get data\n");

    match client.get("https://api.eth-example.com/data").await {
        Ok(response) => {
            println!("✅ Response received!");
            println!("Status: {}", response.status());
            if let Ok(text) = response.text().await {
                println!("Body: {}", text);
            }
        }
        Err(e) => {
            // In demo mode, this will fail because the server doesn't exist
            // In production, it would:
            // 1. Get 402 from server
            // 2. Detect Ethereum address (0x...)
            // 3. Call OutLayer cross-chain withdraw
            // 4. Get Ethereum tx hash
            // 5. Retry with credential
            println!("❌ Error (expected in demo): {}", e);
        }
    }

    println!("\n{}\n", "━".repeat(50));

    // Example 2: Call Solana MPP server
    println!("Example 2: Solana MPP Server");
    println!("Calling: https://api.sol-example.com/query");
    println!("Expected: 402 → Pay to Base58 address → Auto-bridge → Get data\n");

    match client.get("https://api.sol-example.com/query").await {
        Ok(response) => {
            println!("✅ Response received!");
            println!("Status: {}", response.status());
        }
        Err(e) => {
            println!("❌ Error (expected in demo): {}", e);
        }
    }

    println!("\n{}\n", "━".repeat(50));

    // Example 3: Direct bridge usage (for manual payments)
    println!("Example 3: Direct Bridge Usage");
    println!("Manually bridge 1 USDC to Ethereum:\n");

    use mpp_near::bridge::{BridgeClient, BridgeRequest};

    let _bridge = BridgeClient::new("https://bridge.mpp.dev", &api_key);

    let request = BridgeRequest {
        nonce: "demo-nonce-123".to_string(),
        recipient: "0x742d35Cc6634C0532925a3b844Bc4591c494Bc4E".to_string(),
        amount: 1.0,
        token: "usdc".to_string(),
        target_chain: "ethereum".to_string(),
        challenge: None,
    };

    println!("Bridge request:");
    println!("  Recipient: {}", request.recipient);
    println!("  Amount:    {} {}", request.amount, request.token);
    println!("  Chain:     {}", request.target_chain);
    println!("\nNote: Skipping actual payment in demo mode.");
    println!("In production, this would return:");
    println!("  near_tx:   5c571cf2...");
    println!("  target_tx: 0x123abc...");

    // Uncomment to actually bridge (requires real API key and balance):
    // let response = bridge.pay_direct(request).await?;
    // println!("NEAR TX: {}", response.near_tx);
    // println!("ETH TX:  {}", response.target_tx);

    println!("\n{}\n", "━".repeat(50));

    // Budget tracking
    println!("Budget Status:");
    println!("  Spent today: ${:.4}", client.spent_today());
    println!("  Remaining:   ${:.4}", client.remaining_budget());

    println!("\n✨ Example complete!");
    println!("\nKey takeaways:");
    println!("  1. AgentClient auto-detects cross-chain addresses");
    println!("  2. Uses OutLayer for gasless cross-chain payments");
    println!("  3. Returns native chain tx hash for MPP verification");
    println!("  4. Server sees normal payment on its chain");

    Ok(())
}
