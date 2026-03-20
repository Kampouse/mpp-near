//! Basic MPP server example
//!
//! Demonstrates how to use mpp-near to create a payment-gated API server.

use mpp_near::{
    Challenge, RequestData, Credential, Receipt, Problem,
    Method, PaymentRequest, PaymentProof,
    near_intents::NearIntentsMethod,
};

/// Simple test payment method
struct TestMethod;

#[async_trait::async_trait]
impl Method for TestMethod {
    fn id(&self) -> &str {
        "test"
    }
    
    async fn verify(&self, _request: &PaymentRequest, proof: &PaymentProof) -> mpp_near::Result<bool> {
        // Accept test payments
        Ok(proof.proof.starts_with("test_"))
    }
}

#[tokio::main]
async fn main() {
    println!("MPP-NEAR Basic Server Example");
    println!("==============================\n");
    
    // 1. Create a payment request
    let request = RequestData::new("0.001", "merchant.example")
        .currency("USDC");
    
    // 2. Create a payment challenge
    let challenge = Challenge::builder()
        .realm("api.example.com")
        .method("test")
        .intent("charge")
        .request(request)
        .secret(b"test-secret")
        .description("API access")
        .build()
        .unwrap();
    
    println!("1. Challenge created:");
    println!("   ID: {}", challenge.id);
    println!("   Realm: {}", challenge.realm);
    println!("   Method: {}", challenge.method);
    println!("   Intent: {}", challenge.intent);
    println!("   WWW-Authenticate: {}\n", challenge.to_www_authenticate());
    
    // 3. Client creates credential
    let credential = Credential::builder()
        .challenge(&challenge)
        .proof("test_payment_123")
        .source("did:example:user")
        .build()
        .unwrap();
    
    println!("2. Credential created:");
    println!("   Challenge ID: {}", credential.challenge.id);
    println!("   Source: {:?}", credential.source);
    println!("   Authorization: {}\n", credential.to_authorization());
    
    // 4. Server verifies payment
    let method = TestMethod;
    let request = method.extract_request(&challenge).unwrap();
    let proof = method.extract_proof(&credential).unwrap();
    let valid = method.verify(&request, &proof).await.unwrap();
    
    println!("3. Payment verification:");
    println!("   Valid: {}\n", valid);
    
    // 5. Create receipt
    let receipt = Receipt::for_payment(
        &challenge.id,
        credential.source.as_deref(),
        "0.001",
        "USDC",
    );
    
    println!("4. Receipt created:");
    println!("   ID: {}", receipt.id);
    println!("   Status: {}", receipt.status);
    println!("   Payment-Receipt: {}\n", receipt.to_header());
    
    // 6. Example error handling
    let problem = Problem::verification_failed("Signature mismatch");
    println!("5. Example problem:");
    println!("   Type: {}", problem.problem_type);
    println!("   Title: {}", problem.title);
    println!("   Detail: {:?}\n", problem.detail);
    
    println!("✅ Example complete!");
}
