//! Simple test client for MPP payment flow
//!
//! This demonstrates the complete payment flow:
//! 1. Request paid endpoint
//! 2. Receive 402 challenge
//! 3. Pay using OutLayer API
//! 4. Create credential
//! 5. Retry request with payment proof
//! 6. Receive data + receipt

use mpp_near::{
    primitives::{Challenge, Credential, Receipt, RequestData},
    Result,
};
use reqwest::Client;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file
    dotenv::dotenv().ok();

    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║              MPP Payment Flow Test                          ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    // Configuration
    let server_url = env::var("MPP_SERVER_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    let outlayer_api_key = env::var("OUTLAYER_API_KEY").expect("OUTLAYER_API_KEY not set");
    let near_account_id = env::var("NEAR_ACCOUNT_ID").expect("NEAR_ACCOUNT_ID not set");

    println!("📋 Configuration:");
    println!("   Server URL: {}", server_url);
    println!("   Account: {}", near_account_id);
    println!("   API Key: {}...", &outlayer_api_key[..20]);
    println!();

    let client = Client::new();
    let test_url = format!("{}/api/v1/ping", server_url);

    // Step 1: Make initial request (without payment)
    println!("📤 Step 1: Making initial request to {}", test_url);
    let response = client.get(&test_url).send().await?;

    println!("   Status: {}", response.status());
    println!("   Status Code: {}", response.status().as_u16());

    if response.status().as_u16() != 402 {
        println!("   ❌ Expected 402 Payment Required, got {}", response.status());
        println!("   Response: {}", response.text().await?);
        return Ok(());
    }

    // Step 2: Extract challenge from WWW-Authenticate header
    println!("   ✅ Received 402 Payment Required");
    println!();

    let www_auth = response
        .headers()
        .get("www-authenticate")
        .and_then(|v| v.to_str().ok())
        .expect("Missing WWW-Authenticate header");

    println!("📜 Step 2: Parsing challenge from WWW-Authenticate header");
    println!("   Header: {}...", &www_auth[..80]);
    println!();

    let challenge = Challenge::from_www_authenticate(www_auth)?;
    println!("   ✅ Challenge parsed successfully");
    println!("   Challenge ID: {}", challenge.id);
    println!("   Amount: {} (from request)", {
        let request_data = RequestData::decode(&challenge.request)?;
        format!("{} {}", request_data.amount, request_data.currency.as_ref().unwrap_or(&"USD".to_string()))
    });
    println!("   Method: {}", challenge.method);
    println!("   Realm: {}", challenge.realm);
    println!();

    // Step 3: Make payment using OutLayer API
    println!("💳 Step 3: Making payment via OutLayer API");

    let request_data = RequestData::decode(&challenge.request)?;
    let recipient = request_data.recipient;

    println!("   To: {}", recipient);
    println!("   Challenge ID: {}", challenge.id);

    // Prepare OutLayer API request
    let client = reqwest::Client::new();
    let outlayer_url = std::env::var("OUTLAYER_API_URL")
        .unwrap_or_else(|_| "https://api.outlayer.fastnear.com".to_string());

    let currency = request_data.currency.as_deref().unwrap_or("USDC");

    // Determine token ID (USDC uses the long format)
    let token_id = match currency {
        "USDC" => "17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1",
        "USDT" => "usdt.tether-token.near",
        _ => {
            println!("   ❌ Unsupported currency: {}", currency);
            return Ok(());
        }
    };

    // Convert amount to smallest denomination (USDC has 6 decimals)
    let amount_f64 = request_data.amount.parse::<f64>().unwrap_or(0.0);
    let amount_smallest = (amount_f64 * 1_000_000.0) as u64;

    let payload = serde_json::json!({
        "to": recipient,
        "amount": amount_smallest.to_string(),
        "token": token_id,
        "chain": "near"
    });

    println!("   Calling OutLayer API...");
    println!("   URL: {}/wallet/v1/intents/withdraw", outlayer_url);
    println!("   Payload: {}", serde_json::to_string_pretty(&payload).unwrap());

    // Call OutLayer API
    let endpoint = format!("{}/wallet/v1/intents/withdraw", outlayer_url);

    let response = client
        .post(&endpoint)
        .header("Authorization", format!("Bearer {}", outlayer_api_key))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    println!("   Response status: {}", response.status());

    if !response.status().is_success() {
        let error_text = response.text().await?;
        println!("   ❌ OutLayer API error: {}", error_text);
        println!("   💡 Make sure your OutLayer wallet has {} {} balance", request_data.amount, currency);
        return Err(mpp_near::Error::Other(format!("OutLayer API error: {}", error_text)));
    }

    // Parse response to get intent hash
    let response_json: serde_json::Value = response.json().await?;

    println!("   Response: {}", serde_json::to_string_pretty(&response_json).unwrap_or_default());

    // The response might have different field names - try them all
    let tx_hash = response_json
        .get("intent_hash")
        .or_else(|| response_json.get("hash"))
        .or_else(|| response_json.get("request_id"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| mpp_near::Error::Other("OutLayer response missing intent_hash, hash, or request_id".into()))?;

    println!("   ✅ Payment successful!");
    println!("   Transaction/Intent Hash: {}", tx_hash);
    println!();

    // Step 4: Create credential
    println!("🔐 Step 4: Creating payment credential");

    let credential = Credential::builder()
        .challenge(&challenge)
        .proof(&*tx_hash)  // Dereference to get &str
        .source(&near_account_id)
        .build()?;

    println!("   ✅ Credential created");
    println!("   Challenge ID: {}", credential.challenge.id);
    println!("   Proof: {}", tx_hash);
    println!("   Source: {}", near_account_id);
    println!();

        // Step 5: Retry request with credential
        println!("📤 Step 5: Retrying request with payment credential");

        let auth_header = credential.to_authorization();
        println!("   Authorization: {}...", &auth_header[..50]);

        let response = client
            .get(&test_url)
            .header("Authorization", auth_header)
            .send()
            .await?;

        println!("   Status: {}", response.status());

        if response.status().is_success() {
            println!("   ✅ Request successful!");
            println!();

            // Extract receipt headers before consuming response
            let receipt_header = response.headers().get("payment-receipt")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| Receipt::from_header(s));

            // Extract response body
            let body = response.text().await?;
            println!("📦 Response:");
            println!("   {}", body);
            println!();

            // Display receipt if present
            if let Some(receipt) = receipt_header {
                println!("🧾 Receipt:");
                println!("   Receipt ID: {}", receipt.id);
                println!("   Status: {}", receipt.status);
                println!("   Amount: {} {}", receipt.amount, receipt.token);
            }

            println!();
            println!("╔════════════════════════════════════════════════════════════╗");
            println!("║                  ✅ PAYMENT FLOW COMPLETE!                  ║");
            println!("╚════════════════════════════════════════════════════════════╝");
        } else {
            println!("   ❌ Request failed with status: {}", response.status());
            let error_body = response.text().await?;
            println!("   Error: {}", error_body);
        }

    Ok(())
}
