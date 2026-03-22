//! Comprehensive End-to-End Test for MPP-NEAR
//!
//! This test demonstrates the complete payment flow from challenge creation
//! to receipt issuance, including:
//!
//! - Full payment flow (challenge -> credential -> receipt)
//! - Standard NEAR payments
//! - NEAR Intents (gasless) payments
//! - Error handling and edge cases
//! - Spec compliance testing
//! - Mock payment scenarios
//! - Multiple pricing tiers
//! - Challenge expiration
//! - Receipt verification
//!
//! Run with:
//! ```bash
//! cargo test --test end_to_end --features client,server,intents
//! ```
//!
//! For integration testing with actual server:
//! ```bash
//! # Terminal 1: Start server
//! cargo run --example full_server --features server
//!
//! # Terminal 2: Run tests
//! cargo test --example end_to_end_test --features client
//! ```

use std::collections::HashMap;
use mpp_near::{
    near_intents::NearIntentsMethod,
    primitives::{
        Challenge, ChallengeBuilder, Credential, Method, Problem, Receipt, RequestData,
    },
    server::{NearVerifier, VerifierConfig},
    types::{AccountId, Gas, NearAmount},
    Result,
};
use std::time::{Duration, SystemTime};
use tokio::time::{sleep, timeout};
use tracing::{debug, info, warn};

// ============================================================================
// Test Utilities
// ============================================================================

/// Test configuration
struct TestConfig {
    server_url: String,
    recipient_account: AccountId,
    hmac_secret: Vec<u8>,
    challenge_ttl: i64,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            server_url: "http://localhost:3000".to_string(),
            recipient_account: AccountId::new("merchant.near").unwrap(),
            hmac_secret: b"test-hmac-secret-key".to_vec(),
            challenge_ttl: 300, // 5 minutes
        }
    }
}

/// Test result
#[derive(Debug)]
struct TestResult {
    test_name: String,
    passed: bool,
    duration_ms: u64,
    details: String,
}

impl TestResult {
    fn new(test_name: impl Into<String>) -> Self {
        Self {
            test_name: test_name.into(),
            passed: false,
            duration_ms: 0,
            details: String::new(),
        }
    }

    fn success(mut self, details: impl Into<String>) -> Self {
        self.passed = true;
        self.details = details.into();
        self
    }

    fn failure(mut self, details: impl Into<String>) -> Self {
        self.passed = false;
        self.details = details.into();
        self
    }
}

/// Test runner
struct TestRunner {
    results: Vec<TestResult>,
}

impl TestRunner {
    fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    async fn run<F, Fut>(&mut self, test_name: &str, test_fn: F) -> &mut Self
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = TestResult>,
    {
        info!("Running test: {}", test_name);
        let start = std::time::Instant::now();

        let result = test_fn().await;
        let duration = start.elapsed().as_millis() as u64;

        let result = TestResult {
            duration_ms: duration,
            ..result
        };

        self.results.push(result);

        let last = self.results.last().unwrap();
        let status = if last.passed { "✓ PASS" } else { "✗ FAIL" };
        println!("  {} ({} ms) - {}", status, last.duration_ms, last.test_name);
        if !last.passed {
            println!("    Details: {}", last.details);
        }

        self
    }

    fn summary(&self) {
        let total = self.results.len();
        let passed = self.results.iter().filter(|r| r.passed).count();
        let failed = total - passed;
        let total_duration: u64 = self.results.iter().map(|r| r.duration_ms).sum();

        println!("\n╔════════════════════════════════════════════════════════════╗");
        println!("║                   Test Summary                               ║");
        println!("╠════════════════════════════════════════════════════════════╣");
        println!("║  Total Tests: {:<45} ║", total);
        println!("║  Passed:      {:<45} ║", passed);
        println!("║  Failed:      {:<45} ║", failed);
        println!("║  Duration:    {:<45} ║", format!("{} ms", total_duration));
        println!("╚════════════════════════════════════════════════════════════╝");

        if failed > 0 {
            println!("\nFailed tests:");
            for result in &self.results {
                if !result.passed {
                    println!("  ✗ {} - {}", result.test_name, result.details);
                }
            }
        }
    }

    fn all_passed(&self) -> bool {
        self.results.iter().all(|r| r.passed)
    }
}

// ============================================================================
// Core Payment Flow Tests
// ============================================================================

/// Test the complete payment flow: challenge -> credential -> receipt
async fn test_complete_payment_flow(config: &TestConfig) -> TestResult {
    let test_name = "Complete Payment Flow";

    // Step 1: Create challenge
    let request = RequestData::new("0.001", &config.recipient_account.to_string())
        .currency("USDC")
        .token_id("usdc-token-id");

    let challenge = match ChallengeBuilder::new()
        .realm("api.example.com")
        .method("near-intents")
        .intent("charge")
        .request(request)
        .description("Test payment")
        .ttl(config.challenge_ttl)
        .opaque_data({
            let mut map = HashMap::new();
            map.insert("test".to_string(), "opaque".to_string());
            map
        })
        .secret(config.hmac_secret.clone())
        .build()
    {
        Ok(ch) => ch,
        Err(e) => {
            return TestResult::new(test_name).failure(format!("Failed to create challenge: {}", e));
        }
    };

    debug!("Challenge created: {}", challenge.id);

    // Step 2: Verify challenge structure
    if challenge.realm != "api.example.com" {
        return TestResult::new(test_name).failure("Challenge realm mismatch");
    }
    if challenge.method != "near-intents" {
        return TestResult::new(test_name).failure("Challenge method mismatch");
    }
    if challenge.intent != "intent" {
        return TestResult::new(test_name).failure("Challenge intent mismatch");
    }

    // Step 3: Create credential
    let credential = match Credential::builder()
        .challenge(&challenge)
        .proof("test_intent_hash_123")
        .source("did:near:test-account.near")
        .build()
    {
        Ok(cred) => cred,
        Err(e) => {
            return TestResult::new(test_name).failure(format!("Failed to create credential: {}", e));
        }
    };

    debug!("Credential created");

    // Step 4: Verify credential structure
    if !credential.verify_challenge_echo(&challenge) {
        return TestResult::new(test_name).failure("Credential challenge echo failed");
    }

    // Step 5: Create receipt
    let receipt = Receipt::for_payment(
        &challenge.id,
        Some("did:near:test-account.near"),
        "0.001",
        "USDC",
    );

    debug!("Receipt created: {}", receipt.id);

    // Step 6: Verify receipt
    if receipt.challenge_id != challenge.id {
        return TestResult::new(test_name).failure("Receipt challenge ID mismatch");
    }
    if receipt.status != "confirmed" {
        return TestResult::new(test_name).failure("Receipt status not confirmed");
    }

    TestResult::new(test_name).success("Complete flow successful")
}

/// Test challenge binding verification
async fn test_challenge_binding(config: &TestConfig) -> TestResult {
    let test_name = "Challenge Binding Verification";

    let request = RequestData::new("1000", "test-recipient");

    // Create challenge with secret
    let challenge = match ChallengeBuilder::new()
        .realm("test.example.com")
        .method("test")
        .intent("charge")
        .request(request)
        .secret(config.hmac_secret.clone())
        .build()
    {
        Ok(ch) => ch,
        Err(e) => {
            return TestResult::new(test_name).failure(format!("Failed to create challenge: {}", e));
        }
    };

    // Verify with correct secret
    if !challenge.verify_binding(&config.hmac_secret) {
        return TestResult::new(test_name).failure("Failed to verify with correct secret");
    }

    // Verify with wrong secret should fail
    if challenge.verify_binding(b"wrong-secret") {
        return TestResult::new(test_name).failure("Verified with wrong secret (should fail)");
    }

    TestResult::new(test_name).success("Challenge binding working correctly")
}

/// Test credential serialization round-trip
async fn test_credential_roundtrip(config: &TestConfig) -> TestResult {
    let test_name = "Credential Serialization Round-trip";

    let request = RequestData::new("500", "wallet.test");
    let challenge = ChallengeBuilder::new()
        .realm("test.com")
        .method("test")
        .intent("charge")
        .request(request)
        .secret(config.hmac_secret.clone())
        .build()
        .unwrap();

    // Create original credential
    let original = Credential::builder()
        .challenge(&challenge)
        .proof("proof_123")
        .source("did:key:test")
        .build()
        .unwrap();

    // Serialize to Authorization header
    let auth_header = original.to_authorization();
    if !auth_header.starts_with("Payment ") {
        return TestResult::new(test_name).failure("Authorization header missing 'Payment ' prefix");
    }

    // Deserialize from Authorization header
    let parsed = match Credential::from_authorization(&auth_header) {
        Ok(cred) => cred,
        Err(e) => {
            return TestResult::new(test_name).failure(format!("Failed to deserialize: {}", e));
        }
    };

    // Verify equality
    if parsed.challenge.id != original.challenge.id {
        return TestResult::new(test_name).failure("Challenge ID mismatch after round-trip");
    }
    if parsed.payload != original.payload {
        return TestResult::new(test_name).failure("Payload mismatch after round-trip");
    }

    TestResult::new(test_name).success("Credential round-trip successful")
}

/// Test challenge serialization round-trip
async fn test_challenge_roundtrip(config: &TestConfig) -> TestResult {
    let test_name = "Challenge Serialization Round-trip";

    let request = RequestData::new("250", "recipient.test")
        .currency("USD")
        .token_id("usdc.test");

    let original = ChallengeBuilder::new()
        .realm("test.example.com")
        .method("near-intents")
        .intent("charge")
        .request(request)
        .description("Test challenge")
        .expires("2025-01-15T12:05:00Z")
        .opaque_data({
            let mut map = HashMap::new();
            map.insert("data".to_string(), "opaque".to_string());
            map
        })
        .secret(config.hmac_secret.clone())
        .build()
        .unwrap();

    // Serialize to WWW-Authenticate header
    let www_auth = original.to_www_authenticate();
    if !www_auth.starts_with("Payment ") {
        return TestResult::new(test_name).failure("WWW-Authenticate header missing 'Payment ' prefix");
    }

    // Deserialize from WWW-Authenticate header
    let parsed = match Challenge::from_www_authenticate(&www_auth) {
        Ok(ch) => ch,
        Err(e) => {
            return TestResult::new(test_name).failure(format!("Failed to deserialize: {}", e));
        }
    };

    // Verify equality
    if parsed.realm != original.realm {
        return TestResult::new(test_name).failure("Realm mismatch after round-trip");
    }
    if parsed.method != original.method {
        return TestResult::new(test_name).failure("Method mismatch after round-trip");
    }
    if parsed.intent != original.intent {
        return TestResult::new(test_name).failure("Intent mismatch after round-trip");
    }

    TestResult::new(test_name).success("Challenge round-trip successful")
}

/// Test challenge expiration
async fn test_challenge_expiration() -> TestResult {
    let test_name = "Challenge Expiration";

    let request = RequestData::new("100", "test");

    // Create expired challenge
    let expired_challenge = ChallengeBuilder::new()
        .realm("test.com")
        .method("test")
        .intent("charge")
        .request(request.clone())
        .ttl(-1) // Expired 1 second ago
        .secret(b"secret".to_vec())
        .build()
        .unwrap();

    if !expired_challenge.is_expired() {
        return TestResult::new(test_name).failure("Expired challenge not detected");
    }

    // Create valid challenge
    let valid_challenge = ChallengeBuilder::new()
        .realm("test.com")
        .method("test")
        .intent("charge")
        .request(request)
        .ttl(300) // 5 minutes
        .secret(b"secret".to_vec())
        .build()
        .unwrap();

    if valid_challenge.is_expired() {
        return TestResult::new(test_name).failure("Valid challenge detected as expired");
    }

    TestResult::new(test_name).success("Challenge expiration working correctly")
}

/// Test receipt serialization round-trip
async fn test_receipt_roundtrip() -> TestResult {
    let test_name = "Receipt Serialization Round-trip";

    let original = Receipt::builder()
        .challenge_id("test-challenge-123")
        .amount("500")
        .token("USDC")
        .status("confirmed")
        .build()
        .unwrap();

    // Serialize to header
    let header = original.to_header();
    if !header.starts_with("Payment ") {
        return TestResult::new(test_name).failure("Receipt header missing 'Payment ' prefix");
    }

    // Deserialize from header
    let parsed = match Receipt::from_header(&header) {
        Some(r) => r,
        None => {
            return TestResult::new(test_name).failure("Failed to deserialize".to_string());
        }
    };

    // Verify equality
    if parsed.challenge_id != original.challenge_id {
        return TestResult::new(test_name).failure("Challenge ID mismatch after round-trip");
    }
    if parsed.amount != original.amount {
        return TestResult::new(test_name).failure("Amount mismatch after round-trip");
    }
    if parsed.status != original.status {
        return TestResult::new(test_name).failure("Status mismatch after round-trip");
    }

    TestResult::new(test_name).success("Receipt round-trip successful")
}

/// Test problem/error handling
async fn test_problem_handling() -> TestResult {
    let test_name = "Problem Error Handling";

    // Test verification failed problem
    let problem = Problem::verification_failed("Invalid signature");
    if problem.status != 402 {
        return TestResult::new(test_name).failure("Wrong status code for verification failed");
    }
    if problem.problem_type != "https://mpp.dev/problems/verification-failed" {
        return TestResult::new(test_name).failure("Wrong problem type");
    }

    // Test malformed credential problem (instead of invalid request)
    let problem = Problem::malformed_credential("Missing required parameter");
    if problem.status != 402 {
        return TestResult::new(test_name).failure("Wrong status code for malformed credential");
    }

    TestResult::new(test_name).success("Problem handling working correctly")
}

// ============================================================================
// NEAR Intents Tests
// ============================================================================

/// Test NEAR Intents payment method
async fn test_near_intents_method() -> TestResult {
    let test_name = "NEAR Intents Payment Method";

    let method = NearIntentsMethod::new("test-api-key").with_mocks();

    // Verify method ID
    if method.id() != "near-intents" {
        return TestResult::new(test_name).failure("Wrong method ID");
    }

    // Create challenge
    let request = RequestData::new("0.001", "wallet.near")
        .currency("USDC")
        .token_id("usdc-token-id")
        .chain("near");

    let challenge = ChallengeBuilder::new()
        .realm("api.example.com")
        .method("near-intents")
        .intent("charge")
        .request(request)
        .secret(b"test-secret".to_vec())
        .build()
        .unwrap();

    // Create credential with mock proof
    let credential = Credential::builder()
        .challenge(&challenge)
        .proof("test_payment_123")
        .build()
        .unwrap();

    // Verify credential (should accept mock)
    match method.verify_credential(&challenge, &credential).await {
        Ok(verified) => {
            if !verified {
                return TestResult::new(test_name).failure("Mock payment not verified");
            }
        }
        Err(e) => {
            return TestResult::new(test_name).failure(format!("Verification failed: {}", e));
        }
    }

    TestResult::new(test_name).success("NEAR Intents method working correctly")
}

/// Test NEAR Intents request data extraction
async fn test_intents_request_extraction() -> TestResult {
    let test_name = "Intents Request Data Extraction";

    let method = NearIntentsMethod::new("test-api-key");

    let request = RequestData::new("0.001", "wallet.near")
        .currency("USDC")
        .token_id("usdc-token-id")
        .chain("near");

    let challenge = ChallengeBuilder::new()
        .realm("api.example.com")
        .method("near-intents")
        .intent("charge")
        .request(request.clone())
        .secret(b"test-secret".to_vec())
        .build()
        .unwrap();

    // Extract request data
    let extracted = match method.extract_request_data(&challenge) {
        Ok(req) => req,
        Err(e) => {
            return TestResult::new(test_name).failure(format!("Failed to extract: {}", e));
        }
    };

    // Verify extracted data
    if extracted.amount != "0.001" {
        return TestResult::new(test_name).failure("Amount mismatch");
    }
    if extracted.recipient != "wallet.near" {
        return TestResult::new(test_name).failure("Recipient mismatch");
    }
    if extracted.currency != Some("USDC".to_string()) {
        return TestResult::new(test_name).failure("Currency mismatch");
    }

    TestResult::new(test_name).success("Request data extraction successful")
}

// ============================================================================
// Server Verifier Tests
// ============================================================================

/// Test server verifier challenge creation
#[cfg(feature = "server")]
async fn test_server_verifier_challenge() -> TestResult {
    let test_name = "Server Verifier Challenge Creation";

    let config = VerifierConfig {
        rpc_url: "https://rpc.testnet.near.org".to_string(),
        recipient_account: AccountId::new("merchant.near").unwrap(),
        min_amount: NearAmount::from_near(1),
        challenge_ttl: 300,
        confirmations: 12,
        cache_ttl: 3600,
    };

    let verifier = match NearVerifier::new(config) {
        Ok(v) => v,
        Err(e) => {
            return TestResult::new(test_name).failure(format!("Failed to create verifier: {}", e));
        }
    };

    // Create challenge
    let challenge = match verifier.charge("1").await {
        Ok(ch) => ch,
        Err(e) => {
            return TestResult::new(test_name).failure(format!("Failed to create challenge: {}", e));
        }
    };

    // Verify challenge structure
    if challenge.amount.as_near() != 1 {
        return TestResult::new(test_name).failure("Amount mismatch");
    }
    if challenge.recipient.as_str() != "merchant.near" {
        return TestResult::new(test_name).failure("Recipient mismatch");
    }
    if challenge.method != "near" {
        return TestResult::new(test_name).failure("Method mismatch");
    }

    TestResult::new(test_name).success("Verifier challenge creation successful")
}

/// Test server verifier cleanup
#[cfg(feature = "server")]
async fn test_server_verifier_cleanup() -> TestResult {
    let test_name = "Server Verifier Cleanup";

    let config = VerifierConfig {
        rpc_url: "https://rpc.testnet.near.org".to_string(),
        recipient_account: AccountId::new("merchant.near").unwrap(),
        min_amount: NearAmount::from_near(1),
        challenge_ttl: 300,
        confirmations: 12,
        cache_ttl: 3600,
    };

    let verifier = match NearVerifier::new(config) {
        Ok(v) => v,
        Err(e) => {
            return TestResult::new(test_name).failure(format!("Failed to create verifier: {}", e));
        }
    };

    // Create some challenges
    verifier.charge("1").await.unwrap();
    verifier.charge("2").await.unwrap();

    let count_before = verifier.pending_count().await;
    if count_before != 2 {
        return TestResult::new(test_name).failure(format!("Expected 2 pending, got {}", count_before));
    }

    // Cleanup expired challenges (none should be expired)
    verifier.cleanup_expired().await;

    let count_after = verifier.pending_count().await;
    if count_after != 2 {
        return TestResult::new(test_name).failure(format!("Expected 2 pending, got {}", count_after));
    }

    TestResult::new(test_name).success("Verifier cleanup working correctly")
}

// ============================================================================
// Type Validation Tests
// ============================================================================

/// Test AccountId validation
async fn test_account_id_validation() -> TestResult {
    let test_name = "AccountId Validation";

    // Valid account IDs
    let valid_cases = vec![
        "test.near",
        "app.test.near",
        "a.near",
        "my-account.test.near",
        "123.test.near",
    ];

    for account_id in valid_cases {
        if AccountId::new(account_id).is_err() {
            return TestResult::new(test_name).failure(format!("Valid ID rejected: {}", account_id));
        }
    }

    // Invalid account IDs
    let invalid_cases = vec![
        "",
        ".near",
        "near.",
        "test..near",
        "test@near",
        "very-long-account-name-that-exceeds-sixty-three-characters.near",
    ];

    for account_id in invalid_cases {
        if AccountId::new(account_id).is_ok() {
            return TestResult::new(test_name).failure(format!("Invalid ID accepted: {}", account_id));
        }
    }

    TestResult::new(test_name).success("AccountId validation working correctly")
}

/// Test NearAmount conversions
async fn test_near_amount_conversions() -> TestResult {
    let test_name = "NearAmount Conversions";

    // Test from_near
    let near = NearAmount::from_near(1);
    if near.as_near() != 1 {
        return TestResult::new(test_name).failure("NEAR conversion failed");
    }

    // Test from_usdc
    let usdc = NearAmount::from_usdc(100);
    if usdc.0 != 100_000_000 {
        return TestResult::new(test_name).failure("USDC conversion failed");
    }

    // Test large amounts
    let large = NearAmount::from_near(1000);
    if large.as_near() != 1000 {
        return TestResult::new(test_name).failure("Large amount conversion failed");
    }

    TestResult::new(test_name).success("NearAmount conversions working correctly")
}

/// Test Gas conversions
async fn test_gas_conversions() -> TestResult {
    let test_name = "Gas Conversions";

    // Test TERA
    let gas = Gas::tera(100);
    if gas.as_tgas() != 100 {
        return TestResult::new(test_name).failure("TGas conversion failed");
    }

    // Test DEFAULT
    if Gas::DEFAULT.as_tgas() != 100 {
        return TestResult::new(test_name).failure("DEFAULT gas incorrect");
    }

    // Test MAX
    if Gas::MAX.as_tgas() != 300 {
        return TestResult::new(test_name).failure("MAX gas incorrect");
    }

    TestResult::new(test_name).success("Gas conversions working correctly")
}

// ============================================================================
// Edge Cases and Error Handling
// ============================================================================

/// Test invalid challenge handling
async fn test_invalid_challenge() -> TestResult {
    let test_name = "Invalid Challenge Handling";

    // Try to parse invalid WWW-Authenticate header
    let invalid_header = "Payment invalid-json";

    if Challenge::from_www_authenticate(invalid_header).is_ok() {
        return TestResult::new(test_name).failure("Invalid challenge accepted");
    }

    // Try to create credential from invalid Authorization header
    let invalid_auth = "Payment eyJpbnZhbGlkIjogImpzb24ifQ"; // base64 but invalid structure

    if Credential::from_authorization(invalid_auth).is_ok() {
        return TestResult::new(test_name).failure("Invalid credential accepted");
    }

    // Missing Payment prefix
    if Credential::from_authorization("Bearer token").is_ok() {
        return TestResult::new(test_name).failure("Bearer prefix accepted");
    }

    TestResult::new(test_name).success("Invalid challenge handling working correctly")
}

/// Test mismatched challenge and credential
async fn test_mismatched_credential() -> TestResult {
    let test_name = "Mismatched Credential Detection";

    let request1 = RequestData::new("100", "wallet1.near");
    let challenge1 = ChallengeBuilder::new()
        .realm("test.com")
        .method("test")
        .intent("charge")
        .request(request1)
        .secret(b"secret".to_vec())
        .build()
        .unwrap();

    let request2 = RequestData::new("200", "wallet2.near");
    let challenge2 = ChallengeBuilder::new()
        .realm("test.com")
        .method("test")
        .intent("charge")
        .request(request2)
        .secret(b"secret".to_vec())
        .build()
        .unwrap();

    // Create credential with challenge2 but verify against challenge1
    let credential = Credential::builder()
        .challenge(&challenge2)
        .proof("proof")
        .build()
        .unwrap();

    if credential.verify_challenge_echo(&challenge1) {
        return TestResult::new(test_name).failure("Mismatched credential verified");
    }

    TestResult::new(test_name).success("Mismatched credential detection working")
}

/// Test request data encoding/decoding
async fn test_request_data_encoding() -> TestResult {
    let test_name = "Request Data Encoding/Decoding";

    let original = RequestData::new("1000", "wallet.near")
        .currency("USDC")
        .token_id("usdc-token-id")
        .chain("near")
        .method_details(serde_json::json!({"key": "value"}));

    // Encode
    let encoded = match original.encode() {
        Ok(enc) => enc,
        Err(e) => {
            return TestResult::new(test_name).failure(format!("Failed to encode: {}", e));
        }
    };

    // Decode
    let decoded = match RequestData::decode(&encoded) {
        Ok(dec) => dec,
        Err(e) => {
            return TestResult::new(test_name).failure(format!("Failed to decode: {}", e));
        }
    };

    // Verify
    if decoded.amount != original.amount {
        return TestResult::new(test_name).failure("Amount mismatch after encode/decode");
    }
    if decoded.recipient != original.recipient {
        return TestResult::new(test_name).failure("Recipient mismatch after encode/decode");
    }
    if decoded.currency != original.currency {
        return TestResult::new(test_name).failure("Currency mismatch after encode/decode");
    }

    TestResult::new(test_name).success("Request data encoding/decoding successful")
}

// ============================================================================
// Integration Tests (Mock HTTP)
// ============================================================================

/// Test full HTTP flow simulation
async fn test_full_http_flow(config: &TestConfig) -> TestResult {
    let test_name = "Full HTTP Flow Simulation";

    // Step 1: Client makes request
    info!("Step 1: Client makes initial request");

    // Step 2: Server responds with 402 and challenge
    let request = RequestData::new("0.001", &config.recipient_account.to_string())
        .currency("USDC");

    let challenge = ChallengeBuilder::new()
        .realm("api.example.com")
        .method("near-intents")
        .intent("charge")
        .request(request)
        .description("API: /v1/generate")
        .secret(config.hmac_secret.clone())
        .build()
        .unwrap();

    info!("Step 2: Server responds with 402, challenge: {}", challenge.id);

    // Step 3: Client extracts challenge from WWW-Authenticate header
    let www_auth = challenge.to_www_authenticate();
    let extracted_challenge = match Challenge::from_www_authenticate(&www_auth) {
        Ok(ch) => ch,
        Err(e) => {
            return TestResult::new(test_name).failure(format!("Failed to extract challenge: {}", e));
        }
    };

    info!("Step 3: Client extracted challenge");

    // Step 4: Client pays and creates credential
    let credential = Credential::builder()
        .challenge(&extracted_challenge)
        .proof("mock_intent_hash_123")
        .source("did:near:client.near")
        .build()
        .unwrap();

    info!("Step 4: Client created credential");

    // Step 5: Client sends request with Authorization header
    let auth_header = credential.to_authorization();

    info!("Step 5: Client sends request with Authorization");

    // Step 6: Server verifies credential
    if !credential.verify_challenge_echo(&challenge) {
        return TestResult::new(test_name).failure("Challenge echo verification failed");
    }

    if !challenge.verify_binding(&config.hmac_secret) {
        return TestResult::new(test_name).failure("Challenge binding verification failed");
    }

    info!("Step 6: Server verified credential");

    // Step 7: Server creates receipt
    let receipt = Receipt::for_payment(&challenge.id, Some("did:near:client.near"), "0.001", "USDC");

    info!("Step 7: Server created receipt: {}", receipt.id);

    // Step 8: Server responds with 200 and Payment-Receipt header
    let receipt_header = receipt.to_header();

    info!("Step 8: Server responds with 200, receipt: {}", receipt.id);

    // Step 9: Client verifies receipt
    let extracted_receipt = match Receipt::from_header(&receipt_header) {
        Some(r) => r,
        None => {
            return TestResult::new(test_name).failure("Failed to extract receipt".to_string());
        }
    };

    if extracted_receipt.challenge_id != challenge.id {
        return TestResult::new(test_name).failure("Receipt challenge ID mismatch");
    }

    info!("Step 9: Client verified receipt");

    TestResult::new(test_name).success("Full HTTP flow simulation successful")
}

/// Test multiple pricing tiers
async fn test_multiple_pricing_tiers() -> TestResult {
    let test_name = "Multiple Pricing Tiers";

    let pricing = vec![
        ("/health", "0", "USD", "Free"),
        ("/ping", "0.0001", "USDC", "Low cost"),
        ("/analyze", "0.001", "USDC", "Medium cost"),
        ("/generate", "0.01", "USDC", "High cost"),
    ];

    for (endpoint, amount, currency, description) in pricing {
        let request = RequestData::new(amount, "merchant.near").currency(currency);

        let challenge = ChallengeBuilder::new()
            .realm("api.example.com")
            .method("near-intents")
            .intent("charge")
            .request(request)
            .description(description)
            .secret(b"secret".to_vec())
            .build()
            .unwrap();

        let credential = Credential::builder()
            .challenge(&challenge)
            .proof("mock_proof")
            .build()
            .unwrap();

        let receipt = Receipt::for_payment(&challenge.id, None, amount, currency);

        if receipt.amount != amount {
            return TestResult::new(test_name).failure(format!(
                "Receipt amount mismatch for {}: expected {}, got {}",
                endpoint, amount, receipt.amount
            ));
        }

        info!("✓ {} - {} {} ({})", endpoint, amount, currency, description);
    }

    TestResult::new(test_name).success("Multiple pricing tiers working correctly")
}

// ============================================================================
// Main Test Runner
// ============================================================================

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file
    dotenv::dotenv().ok();

    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║          MPP-NEAR End-to-End Test Suite                   ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    let config = TestConfig::default();
    let mut runner = TestRunner::new();

    // Core Payment Flow Tests
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║  Core Payment Flow Tests                                    ║");
    println!("╚════════════════════════════════════════════════════════════╝");

    runner.run("Complete Payment Flow", || test_complete_payment_flow(&config)).await;
    runner.run("Challenge Binding Verification", || test_challenge_binding(&config)).await;
    runner.run("Credential Serialization Round-trip", || test_credential_roundtrip(&config)).await;
    runner.run("Challenge Serialization Round-trip", || test_challenge_roundtrip(&config)).await;
    runner.run("Challenge Expiration", || test_challenge_expiration()).await;
    runner.run("Receipt Serialization Round-trip", || test_receipt_roundtrip()).await;
    runner.run("Problem Error Handling", || test_problem_handling()).await;

    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║  NEAR Intents Tests                                        ║");
    println!("╚════════════════════════════════════════════════════════════╝");

    runner.run("NEAR Intents Payment Method", || test_near_intents_method()).await;
    runner.run("Intents Request Data Extraction", || test_intents_request_extraction()).await;

    #[cfg(feature = "server")]
    {
        println!("\n╔════════════════════════════════════════════════════════════╗");
        println!("║  Server Verifier Tests                                    ║");
        println!("╚════════════════════════════════════════════════════════════╝");

        runner.run("Server Verifier Challenge Creation", || test_server_verifier_challenge()).await;
        runner.run("Server Verifier Cleanup", || test_server_verifier_cleanup()).await;
    }

    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║  Type Validation Tests                                      ║");
    println!("╚════════════════════════════════════════════════════════════╝");

    runner.run("AccountId Validation", || test_account_id_validation()).await;
    runner.run("NearAmount Conversions", || test_near_amount_conversions()).await;
    runner.run("Gas Conversions", || test_gas_conversions()).await;

    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║  Edge Cases and Error Handling                             ║");
    println!("╚════════════════════════════════════════════════════════════╝");

    runner.run("Invalid Challenge Handling", || test_invalid_challenge()).await;
    runner.run("Mismatched Credential Detection", || test_mismatched_credential()).await;
    runner.run("Request Data Encoding/Decoding", || test_request_data_encoding()).await;

    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║  Integration Tests                                         ║");
    println!("╚════════════════════════════════════════════════════════════╝");

    runner.run("Full HTTP Flow Simulation", || test_full_http_flow(&config)).await;
    runner.run("Multiple Pricing Tiers", || test_multiple_pricing_tiers()).await;

    // Print summary
    runner.summary();

    // Exit with appropriate code
    if runner.all_passed() {
        println!("\n✅ All tests passed!");
        Ok(())
    } else {
        println!("\n❌ Some tests failed!");
        Err(mpp_near::Error::Other("Tests failed".to_string()))
    }
}
