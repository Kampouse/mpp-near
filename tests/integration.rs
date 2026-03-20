//! Integration tests for mpp-near

use mpp_near::{Challenge, RequestData, Credential, Receipt, Problem, BodyDigest};

#[test]
fn test_full_flow() {
    // 1. Server creates payment request
    let request = RequestData::new("1000", "acct_123")
        .currency("usd");
    
    // 2. Server creates challenge
    let challenge = Challenge::builder()
        .realm("api.example.com")
        .method("near-intents")
        .intent("charge")
        .request(request)
        .secret(b"test-secret-key".to_vec())
        .description("API access")
        .build()
        .expect("valid challenge");
    
    // 3. Client receives 402 with WWW-Authenticate header
    let www_auth = challenge.to_www_authenticate();
    assert!(www_auth.starts_with("Payment "));
    assert!(www_auth.contains("realm=\"api.example.com\""));
    assert!(www_auth.contains("method=\"near-intents\""));
    assert!(www_auth.contains("intent=\"charge\""));
    
    // 4. Client pays and creates credential
    let credential = Credential::builder()
        .challenge(&challenge)
        .proof("intent_hash_123")
        .source("did:near:user.near")
        .build()
        .expect("valid credential");
    
    // 5. Client retries with Authorization header (base64url JSON)
    let auth = credential.to_authorization();
    assert!(auth.starts_with("Payment "));
    
    // 6. Server verifies credential
    assert!(credential.verify_challenge_echo(&challenge));
    
    // 7. Server verifies challenge binding (stateless)
    assert!(challenge.verify_binding(b"test-secret-key"));
    
    // 8. Server creates receipt
    let receipt = Receipt::for_payment(&challenge.id, None, "1000", "usd");
    
    // 9. Server returns 200 with Payment-Receipt header
    let receipt_header = receipt.to_header();
    assert!(receipt_header.starts_with("Payment "));
}

#[test]
fn test_challenge_roundtrip() {
    let request = RequestData::new("0.001", "wallet.near")
        .currency("USDC");
    
    let original = Challenge::builder()
        .realm("api.example.com")
        .method("near-intents")
        .intent("charge")
        .request(request)
        .secret(b"test-secret".to_vec())
        .build()
        .unwrap();
    
    let header = original.to_www_authenticate();
    let parsed = Challenge::from_www_authenticate(&header).unwrap();
    
    assert_eq!(parsed.realm, original.realm);
    assert_eq!(parsed.method, original.method);
    assert_eq!(parsed.intent, original.intent);
    assert_eq!(parsed.request, original.request);
}

#[test]
fn test_credential_roundtrip() {
    let request = RequestData::new("1000", "acct_123");
    
    let challenge = Challenge::builder()
        .realm("api.example.com")
        .method("test")
        .intent("charge")
        .request(request)
        .secret(b"test-secret".to_vec())
        .build()
        .unwrap();
    
    let original = Credential::builder()
        .challenge(&challenge)
        .proof("test_proof_123")
        .source("did:key:z6MkhaXg")
        .build()
        .unwrap();
    
    let auth = original.to_authorization();
    let parsed = Credential::from_authorization(&auth).unwrap();
    
    assert_eq!(parsed.challenge.id, original.challenge.id);
    assert_eq!(parsed.challenge.realm, original.challenge.realm);
    assert_eq!(parsed.payload, original.payload);
}

#[test]
fn test_receipt_roundtrip() {
    let original = Receipt::builder()
        .challenge_id("test-id-123")
        .amount("1000")
        .token("USDC")
        .status("confirmed")
        .build()
        .unwrap();
    
    let header = original.to_header();
    let parsed = Receipt::from_header(&header).unwrap();
    
    assert_eq!(parsed.challenge_id, original.challenge_id);
    assert_eq!(parsed.status, original.status);
}

#[test]
fn test_body_digest() {
    let body = b"test body";
    let digest = BodyDigest::sha256(body);
    
    assert!(digest.verify(body));
    assert!(!digest.verify(b"different body"));
    
    let header = digest.to_header();
    let parsed = BodyDigest::from_header(&header).unwrap();
    assert!(parsed.verify(body));
}

#[test]
fn test_problem_creation() {
    let problem = Problem::verification_failed("Invalid signature");
    assert_eq!(problem.status, 402);
    assert!(problem.detail.as_ref().unwrap().contains("Invalid signature"));
}

#[test]
fn test_challenge_expiration() {
    let request = RequestData::new("1", "wallet.near");
    
    let challenge = Challenge::builder()
        .realm("test")
        .method("test")
        .intent("charge")
        .request(request)
        .ttl(10)
        .build()
        .unwrap();
    
    assert!(!challenge.is_expired());
}

#[test]
fn test_hmac_binding() {
    let secret = b"my-secret-key";
    let request = RequestData::new("1000", "acct_123");
    
    let challenge = Challenge::builder()
        .realm("api.example.com")
        .method("test")
        .intent("charge")
        .request(request)
        .secret(secret.to_vec())
        .build()
        .unwrap();
    
    // Verify with correct secret
    assert!(challenge.verify_binding(secret));
    
    // Verify with wrong secret fails
    assert!(!challenge.verify_binding(b"wrong-secret"));
}

#[test]
fn test_request_data_encoding() {
    let request = RequestData::new("1000", "acct_123")
        .currency("usd")
        .token_id("usdc.near")
        .chain("near");
    
    let encoded = request.encode().unwrap();
    let decoded = RequestData::decode(&encoded).unwrap();
    
    assert_eq!(decoded.amount, "1000");
    assert_eq!(decoded.recipient, "acct_123");
    assert_eq!(decoded.currency, Some("usd".to_string()));
    assert_eq!(decoded.token_id, Some("usdc.near".to_string()));
    assert_eq!(decoded.chain, Some("near".to_string()));
}

#[test]
fn test_spec_compliance() {
    // Test that we can create spec-compliant challenges
    let request = RequestData::new("1000", "acct_123")
        .currency("usd");
    
    let challenge = Challenge::builder()
        .realm("api.example.com")
        .method("invoice")
        .intent("charge")
        .request(request)
        .expires("2025-01-15T12:05:00Z")
        .id("test-id-123")
        .build()
        .unwrap();
    
    // Verify all required fields
    assert!(!challenge.id.is_empty());
    assert!(!challenge.realm.is_empty());
    assert!(!challenge.method.is_empty());
    assert!(!challenge.intent.is_empty());
    assert!(!challenge.request.is_empty());
    
    // Verify WWW-Authenticate format
    let header = challenge.to_www_authenticate();
    assert!(header.contains("id="));
    assert!(header.contains("realm="));
    assert!(header.contains("method="));
    assert!(header.contains("intent="));
    assert!(header.contains("request="));
}
