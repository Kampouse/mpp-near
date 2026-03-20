//! NEAR Intents MPP server example

use mpp_near::{
    Challenge, RequestData, Credential, Receipt, Problem,
    Method, near_intents::NearIntentsMethod,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("MPP-NEAR Intents Server Example");
    println!("================================\n");
    
    // Configuration
    let api_key = std::env::var("OUTLAYER_API_KEY")
        .unwrap_or_else(|_| "wk_test_key".to_string());
    let recipient = "5c571cf253c3edb672df980cc56078f2c455b972cc01ac34af51e95967ba6edb";
    let secret = b"my-hmac-secret-key";
    
    // 1. Create payment method
    let method = NearIntentsMethod::new(&api_key).with_mocks();
    println!("1. Payment method: {}", method.id());
    
    // 2. Create payment request
    let request = RequestData::new("0.001", recipient)
        .currency("USDC")
        .token_id("17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1")
        .chain("near");
    
    // 3. Create challenge
    let challenge = Challenge::builder()
        .realm("api.example.com")
        .method("near-intents")
        .intent("charge")
        .request(request)
        .description("API: /v1/generate")
        .secret(secret.to_vec())
        .build()?;
    
    println!("2. Challenge created:");
    println!("   ID: {}", challenge.id);
    println!("   Realm: {}", challenge.realm);
    println!("   Method: {}", challenge.method);
    println!("   Intent: {}", challenge.intent);
    println!("   Expires: {:?}\n", challenge.expires);
    
    // 4. Show WWW-Authenticate header
    let www_auth = challenge.to_www_authenticate();
    println!("3. WWW-Authenticate header:");
    println!("   {}\n", www_auth);
    
    // 5. Client pays and creates credential
    let credential = Credential::builder()
        .challenge(&challenge)
        .proof("test_intent_hash_123")
        .source("did:near:user.near")
        .build()?;
    
    println!("4. Credential created:");
    println!("   Challenge ID: {}", credential.challenge.id);
    println!("   Source: {:?}\n", credential.source);
    
    // 6. Show Authorization header (base64url JSON)
    let auth = credential.to_authorization();
    println!("5. Authorization header:");
    println!("   {}\n", auth);
    
    // 7. Verify credential
    let valid = method.verify_credential(&challenge, &credential).await?;
    println!("6. Payment verification: {}\n", valid);
    
    // 8. Create receipt
    let receipt = Receipt::for_payment(&challenge.id, credential.source.as_deref(), "0.001", "USDC");
    println!("7. Receipt created:");
    println!("   ID: {}", receipt.id);
    println!("   Status: {}", receipt.status);
    println!("   Payment-Receipt: {}\n", receipt.to_header());
    
    // 9. Show error example
    let problem = Problem::verification_failed("Invalid signature");
    println!("8. Example error:");
    println!("   Type: {}", problem.problem_type);
    println!("   Title: {}", problem.title);
    println!("   Status: {}", problem.status);
    println!("   Detail: {:?}\n", problem.detail);
    
    // 10. Show full HTTP flow
    println!("9. Full HTTP Flow:");
    println!("   ┌─────────────────────────────────────┐");
    println!("   │ Client                          Server │");
    println!("   ├─────────────────────────────────────┤");
    println!("   │ GET /api/v1/generate                │");
    println!("   │────────────────────────────────────>│");
    println!("   │                                     │");
    println!("   │ 402 Payment Required                │");
    println!("   │ WWW-Authenticate: Payment ...       │");
    println!("   │<────────────────────────────────────│");
    println!("   │                                     │");
    println!("   │ [Client pays via NEAR Intents]      │");
    println!("   │                                     │");
    println!("   │ GET /api/v1/generate                │");
    println!("   │ Authorization: Payment eyJ...       │");
    println!("   │────────────────────────────────────>│");
    println!("   │                                     │");
    println!("   │ 200 OK                              │");
    println!("   │ Payment-Receipt: Payment eyJ...     │");
    println!("   │<────────────────────────────────────│");
    println!("   └─────────────────────────────────────┘\n");
    
    // 11. Pricing table
    println!("10. Pricing Table:");
    let pricing = vec![
        ("/v1/generate", "0.01", "USDC", "Image generation"),
        ("/v1/search", "0.001", "USDC", "Web search"),
        ("/v1/analyze", "0.005", "USDC", "Data analysis"),
        ("/v1/chat", "0.0001", "USDC", "Chat completion"),
    ];
    
    for (endpoint, amount, token, desc) in pricing {
        println!("   {} - {} {} ({})", endpoint, amount, token, desc);
    }
    
    println!("\n✅ Example complete!");
    println!("\nTo use in production:");
    println!("1. Set OUTLAYER_API_KEY environment variable");
    println!("2. Remove .with_mocks() from NearIntentsMethod");
    println!("3. Use middleware::PaymentLayer in your Axum router");
    println!("4. Store HMAC secret securely (e.g., environment variable)");
    
    Ok(())
}
