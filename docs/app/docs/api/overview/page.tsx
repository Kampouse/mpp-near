export default function APIOverviewPage() {
  const sections = [
    { id: "overview", title: "Overview", icon: "📖" },
    { id: "installation", title: "Installation", icon: "📦" },
    { id: "types", title: "Core Types", icon: "🧩" },
    { id: "challenge", title: "Challenge", icon: "🔐" },
    { id: "request-data", title: "RequestData", icon: "📋" },
    { id: "credential", title: "Credential", icon: "✓" },
    { id: "receipt", title: "Receipt", icon: "📝" },
    { id: "payment-proof", title: "PaymentProof", icon: "🔑" },
    { id: "body-digest", title: "Body Digest", icon: "🔗" },
    { id: "headers", title: "Headers", icon: "📡" },
    { id: "verification", title: "Verification", icon: "🔍" },
    { id: "custom-methods", title: "Custom Methods", icon: "⚡" },
    { id: "errors", title: "Errors", icon: "⚠️" },
    { id: "constants", title: "Constants", icon: "📌" },
    { id: "examples", title: "Complete Example", icon: "💻" },
    { id: "middleware", title: "Axum Middleware", icon: "⚙️" },
  ]

  return (
    <div className="flex gap-8">
      {/* Main Content */}
      <div className="flex-1 min-w-0">
        <h1 className="text-4xl font-bold mb-4 text-slate-900 dark:text-slate-50">Rust SDK Documentation</h1>
        <p className="text-xl text-slate-700 dark:text-slate-300 mb-8">
          Complete reference for MPP-NEAR Rust SDK. Build payment-gated APIs with type-safe primitives.
        </p>

        <div id="overview" className="bg-white dark:bg-slate-800 border-l-4 rounded-lg p-6 mb-8 shadow-sm" style={{borderColor: '#00C08B'}}>
          <h3 className="font-semibold text-slate-900 dark:text-slate-100 mb-3 text-lg">Features</h3>
          <ul className="space-y-2 text-slate-700 dark:text-slate-300">
            <li className="flex items-start gap-2">
              <span className="mt-0.5" style={{color: '#00C08B'}}>✅</span>
              <span><strong className="text-slate-900 dark:text-slate-100">Stateless verification</strong> — HMAC-based challenge binding</span>
            </li>
            <li className="flex items-start gap-2">
              <span className="mt-0.5" style={{color: '#00C08B'}}>✅</span>
              <span><strong className="text-slate-900 dark:text-slate-100">Type-safe API</strong> — Builder patterns with compile-time checks</span>
            </li>
            <li className="flex items-start gap-2">
              <span className="mt-0.5" style={{color: '#00C08B'}}>✅</span>
              <span><strong className="text-slate-900 dark:text-slate-100">MPP-1.0 compliant</strong> — Spec-conformant implementation</span>
            </li>
            <li className="flex items-start gap-2">
              <span className="mt-0.5" style={{color: '#00C08B'}}>✅</span>
              <span><strong className="text-slate-900 dark:text-slate-100">Axum integration</strong> — Middleware & extractors included</span>
            </li>
            <li className="flex items-start gap-2">
              <span className="mt-0.5" style={{color: '#00C08B'}}>✅</span>
              <span><strong className="text-slate-900 dark:text-slate-100">Extensible</strong> — Custom payment methods via traits</span>
            </li>
          </ul>
        </div>

        <h2 id="installation" className="text-2xl font-bold mb-4 text-slate-900 dark:text-slate-100 scroll-mt-20">Installation</h2>
        <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4 mb-8">
          <pre className="text-sm" style={{color: '#00C08B'}}>
            <code>{`# Cargo.toml
[dependencies]
mpp-near = { git = "https://github.com/kampouse/mpp-near", features = ["server"] }
tokio = { version = "1", features = ["full"] }
axum = "0.7"
serde = { version = "1", features = ["derive"] }
serde_json = "1"`}</code>
          </pre>
        </div>

        <h2 id="types" className="text-2xl font-bold mb-6 text-slate-900 dark:text-slate-100 scroll-mt-20">Core Types</h2>
        <div className="grid md:grid-cols-2 gap-4 mb-8">
          <div className="bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg p-4">
            <h4 className="font-semibold mb-2 text-slate-900 dark:text-slate-100">Challenge</h4>
            <p className="text-sm text-slate-600 dark:text-slate-400">Payment requirements returned with 402</p>
          </div>
          <div className="bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg p-4">
            <h4 className="font-semibold mb-2 text-slate-900 dark:text-slate-100">RequestData</h4>
            <p className="text-sm text-slate-600 dark:text-slate-400">Payment amount and details</p>
          </div>
          <div className="bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg p-4">
            <h4 className="font-semibold mb-2 text-slate-900 dark:text-slate-100">Credential</h4>
            <p className="text-sm text-slate-600 dark:text-slate-400">Proof of payment from client</p>
          </div>
          <div className="bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg p-4">
            <h4 className="font-semibold mb-2 text-slate-900 dark:text-slate-100">Receipt</h4>
            <p className="text-sm text-slate-600 dark:text-slate-400">Payment confirmation</p>
          </div>
          <div className="bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg p-4">
            <h4 className="font-semibold mb-2 text-slate-900 dark:text-slate-100">Problem</h4>
            <p className="text-sm text-slate-600 dark:text-slate-400">RFC 9457 error responses</p>
          </div>
          <div className="bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg p-4">
            <h4 className="font-semibold mb-2 text-slate-900 dark:text-slate-100">Verifier</h4>
            <p className="text-sm text-slate-600 dark:text-slate-400">Payment verification trait</p>
          </div>
        </div>

        <h2 id="challenge" className="text-2xl font-bold mb-4 text-slate-900 dark:text-slate-100 scroll-mt-20">Challenge Type</h2>

        <div className="mb-6">
          <h3 className="text-lg font-semibold mb-3 text-slate-900 dark:text-slate-100">Fields</h3>
          <div className="bg-slate-50 dark:bg-slate-900 rounded-lg p-4 mb-4 overflow-x-auto">
            <pre className="text-sm text-slate-700 dark:text-slate-300"><code>{`pub struct Challenge {
    pub id: String,              // Unique identifier (HMAC-derived)
    pub realm: String,           // Protection space (e.g., "api.example.com")
    pub method: String,          // Payment method (e.g., "near-intents")
    pub intent: String,          // Intent: "charge" | "session" | ...
    pub request: String,         // Base64url-encoded RequestData
    pub expires: Option<String>, // RFC 3339 timestamp
    pub digest: Option<String>,  // RFC 9530 body digest
    pub description: Option<String>, // Human-readable description
    pub opaque: Option<String>,  // Server correlation data
}`}</code></pre>
          </div>
        </div>

        <div className="mb-6">
          <h3 className="text-lg font-semibold mb-3 text-slate-900 dark:text-slate-100">ChallengeBuilder</h3>
          <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4 mb-4">
            <pre className="text-sm" style={{color: '#00C08B'}}>
              <code>{`use mpp_near::{Challenge, RequestData};

let challenge = Challenge::builder()
    .realm("api.example.com")
    .method("near-intents")
    .intent("charge")
    .request(RequestData::new("0.001", "wallet.near"))
    .currency("USDC")
    .expires(300)           // TTL in seconds (default: 300)
    .secret(b"your-hmac-secret")
    .description("Image generation API")
    .build()?;`}</code>
            </pre>
          </div>

          <h4 className="text-md font-semibold mb-2 text-slate-900 dark:text-slate-100 mt-6">Builder Methods</h4>
          <div className="bg-slate-50 dark:bg-slate-900 rounded-lg p-4">
            <table className="w-full text-sm">
              <thead>
                <tr className="border-b border-slate-200 dark:border-slate-700">
                  <th className="text-left py-2 text-slate-900 dark:text-slate-100">Method</th>
                  <th className="text-left py-2 text-slate-900 dark:text-slate-100">Description</th>
                </tr>
              </thead>
              <tbody className="text-slate-700 dark:text-slate-300">
                <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">realm()</code></td><td>Set protection space</td></tr>
                <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">method()</code></td><td>Set payment method</td></tr>
                <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">intent()</code></td><td>Set payment intent</td></tr>
                <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">request()</code></td><td>Set RequestData</td></tr>
                <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">expires()</code></td><td>Set TTL (seconds)</td></tr>
                <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">digest()</code></td><td>Bind to request body</td></tr>
                <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">description()</code></td><td>Human-readable description</td></tr>
                <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">opaque()</code></td><td>Correlation data</td></tr>
                <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">secret()</code></td><td>HMAC secret for signing</td></tr>
              </tbody>
            </table>
          </div>
        </div>

        <h2 id="request-data" className="text-2xl font-bold mb-4 text-slate-900 dark:text-slate-100 scroll-mt-20">RequestData Type</h2>

        <div className="mb-6">
          <h3 className="text-lg font-semibold mb-3 text-slate-900 dark:text-slate-100">Fields</h3>
          <div className="bg-slate-50 dark:bg-slate-900 rounded-lg p-4 mb-4 overflow-x-auto">
            <pre className="text-sm text-slate-700 dark:text-slate-300"><code>{`pub struct RequestData {
    pub amount: String,                        // Decimal amount
    pub currency: Option<String>,               // Token symbol (e.g., "USDC")
    pub token_id: Option<String>,               // Contract address
    pub recipient: String,                     // Recipient address
    pub chain: Option<String>,                  // Blockchain/network
    pub method_details: Option<Value>,          // Method-specific data
    pub extra: HashMap<String, Value>,         // Additional fields
}`}</code></pre>
          </div>
        </div>

        <div className="mb-6">
          <h3 className="text-lg font-semibold mb-3 text-slate-900 dark:text-slate-100">Methods</h3>
          <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4 mb-4">
            <pre className="text-sm" style={{color: '#00C08B'}}>
              <code>{`use mpp_near::RequestData;

// Create with required fields
let request = RequestData::new("0.001", "wallet.near");

// Builder pattern for optional fields
let request = RequestData::new("0.001", "wallet.near")
    .currency("USDC")
    .token_id("usdc.contract.near")
    .chain("near")
    .method_details(json!({"swap": "auto"}))
    .extra("max_slippage", json!("0.5"))
    .encode()?;  // Encode to base64url`}</code>
            </pre>
          </div>
        </div>

        <h2 id="credential" className="text-2xl font-bold mb-4 text-slate-900 dark:text-slate-100 scroll-mt-20">Credential Type</h2>

        <div className="mb-6">
          <h3 className="text-lg font-semibold mb-3 text-slate-900 dark:text-slate-100">Fields</h3>
          <div className="bg-slate-50 dark:bg-slate-900 rounded-lg p-4 mb-4 overflow-x-auto">
            <pre className="text-sm text-slate-700 dark:text-slate-300"><code>{`pub struct Credential {
    pub challenge: ChallengeEcho,    // Challenge reference
    pub proof: PaymentProof,         // Method-specific proof
    pub expires: String,             // RFC 3339 timestamp
    pub signature: String,           // Client signature
}`}</code></pre>
          </div>
        </div>

        <div className="mb-6">
          <h3 className="text-lg font-semibold mb-3 text-slate-900 dark:text-slate-100">Creating Credentials (Client-side)</h3>
          <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4 mb-4">
            <pre className="text-sm" style={{color: '#00C08B'}}>
              <code>{`use mpp_near::Credential;

let credential = Credential::builder()
    .challenge(&challenge)
    .proof("intent_hash_from_payment")
    .sign(client_private_key)?
    .build()?;`}</code>
            </pre>
          </div>
        </div>

        <div className="mb-6">
          <h3 className="text-lg font-semibold mb-3 text-slate-900 dark:text-slate-100">Verifying Credentials (Server-side)</h3>
          <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4 mb-4">
            <pre className="text-sm" style={{color: '#00C08B'}}>
              <code>{`use mpp_near::{Credential, VerificationResult};

// From Authorization header
let credential = Credential::from_authorization(auth_header)?;

// Verify challenge binding
match credential.verify(&challenge) {
    VerificationResult::Valid => {
        // Payment verified successfully
    },
    VerificationResult::Invalid(reason) => {
        // Payment verification failed
        return Err(reason);
    },
}`}</code>
            </pre>
          </div>
        </div>

        <h2 id="receipt" className="text-2xl font-bold mb-4 text-slate-900 dark:text-slate-100 scroll-mt-20">Receipt Type</h2>

        <div className="mb-6">
          <p className="mb-4 text-slate-700 dark:text-slate-300">
            Issue receipts to confirm successful payments. Return via <code className="bg-slate-100 dark:bg-slate-800 px-2 py-0.5 rounded font-mono text-sm">Payment-Receipt</code> header.
          </p>
          <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4 mb-4">
            <pre className="text-sm" style={{color: '#00C08B'}}>
              <code>{`use mpp_near::Receipt;

let receipt = Receipt::builder()
    .challenge_id(&challenge.id)
    .payer(Some("user.near"))
    .amount("0.001")
    .currency("USDC")
    .intent("charge")
    .build()?;

// Convert to header value
let header_value = receipt.to_header();
response.headers.insert("Payment-Receipt", header_value);`}</code>
            </pre>
          </div>

          <h4 className="text-md font-semibold mb-2 text-slate-900 dark:text-slate-100 mt-6">Receipt Fields</h4>
          <div className="bg-slate-50 dark:bg-slate-900 rounded-lg p-4">
            <table className="w-full text-sm">
              <thead>
                <tr className="border-b border-slate-200 dark:border-slate-700">
                  <th className="text-left py-2 text-slate-900 dark:text-slate-100">Field</th>
                  <th className="text-left py-2 text-slate-900 dark:text-slate-100">Type</th>
                  <th className="text-left py-2 text-slate-900 dark:text-slate-100">Description</th>
                </tr>
              </thead>
              <tbody className="text-slate-700 dark:text-slate-300">
                <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">challenge_id</code></td><td>String</td><td>Reference to challenge</td></tr>
                <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">payer</code></td><td>Option&lt;String&gt;</td><td>Payer address</td></tr>
                <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">amount</code></td><td>String</td><td>Amount paid</td></tr>
                <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">currency</code></td><td>String</td><td>Token symbol</td></tr>
                <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">intent</code></td><td>String</td><td>Payment intent</td></tr>
                <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">timestamp</code></td><td>String</td><td>ISO 8601 timestamp</td></tr>
              </tbody>
            </table>
          </div>
        </div>

        <h2 id="payment-proof" className="text-2xl font-bold mb-4 text-slate-900 dark:text-slate-100 scroll-mt-20">PaymentProof Type</h2>

        <div className="mb-6">
          <p className="mb-4 text-slate-700 dark:text-slate-300">
            Payment proof contains method-specific payment data from the credential.
          </p>
          <div className="bg-slate-50 dark:bg-slate-900 rounded-lg p-4 mb-4 overflow-x-auto">
            <pre className="text-sm text-slate-700 dark:text-slate-300"><code>{`pub struct PaymentProof {
    pub proof: String,                          // Transaction hash
    pub account: Option<String>,                  // Payer account
    pub signature: Option<String>,               // Transaction signature
    pub public_key: Option<String>,              // Payer public key
    pub extra: HashMap<String, Value>,           // Additional data
}`}</code></pre>
          </div>

          <h4 className="text-md font-semibold mb-2 text-slate-900 dark:text-slate-100 mt-6">Creating PaymentProof</h4>
          <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4 mb-4">
            <pre className="text-sm" style={{color: '#00C08B'}}>
              <code>{`use mpp_near::PaymentProof;

// Simple proof
let proof = PaymentProof::new("intent_hash_here");

// From credential payload (JSON)
let proof = PaymentProof::from_payload(&credential_json)?;`}</code>
            </pre>
          </div>
        </div>

        <h2 id="body-digest" className="text-2xl font-bold mb-4 text-slate-900 dark:text-slate-100 scroll-mt-20">Body Digest (RFC 9530)</h2>

        <div className="mb-6">
          <p className="mb-4 text-slate-700 dark:text-slate-300">
            Bind challenges to request bodies to prevent tampering. Clients cannot modify the body after receiving a challenge.
          </p>
          <div className="bg-slate-50 dark:bg-slate-900 rounded-lg p-4 mb-4 overflow-x-auto">
            <pre className="text-sm text-slate-700 dark:text-slate-300"><code>{`pub struct BodyDigest {
    pub algorithm: DigestAlgorithm,  // Sha256 or Sha512
    pub hash: String,               // Base64-encoded hash
}

pub enum DigestAlgorithm {
    Sha256,  // SHA-256
    Sha512,  // SHA-512
}`}</code></pre>
          </div>

          <h4 className="text-md font-semibold mb-2 text-slate-900 dark:text-slate-100 mt-6">Creating Body Digests</h4>
          <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4 mb-4">
            <pre className="text-sm" style={{color: '#00C08B'}}>
              <code>{`use mpp_near::BodyDigest;

// From request body
let body = b"{\\"prompt\\": \\"generate image\\"}";
let digest = BodyDigest::sha256(body);

// Format: "sha-256=:base64hash:"
let digest_header = digest.to_header();

// Add to challenge
let challenge = Challenge::builder()
    .request(RequestData::new("0.001", "wallet.near"))
    .body_digest(digest)
    .build()?;`}</code>
            </pre>
          </div>

          <h4 className="text-md font-semibold mb-2 text-slate-900 dark:text-slate-100 mt-6">Supported Algorithms</h4>
          <div className="bg-slate-50 dark:bg-slate-900 rounded-lg p-4">
            <table className="w-full text-sm">
              <thead>
                <tr className="border-b border-slate-200 dark:border-slate-700">
                  <th className="text-left py-2 text-slate-900 dark:text-slate-100">Algorithm</th>
                  <th className="text-left py-2 text-slate-900 dark:text-slate-100">Use Case</th>
                </tr>
              </thead>
              <tbody className="text-slate-700 dark:text-slate-300">
                <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">sha-256</code></td><td>Default, fast</td></tr>
                <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">sha-512</code></td><td>Higher security</td></tr>
              </tbody>
            </table>
          </div>
        </div>

        <h2 id="headers" className="text-2xl font-bold mb-4 text-slate-900 dark:text-slate-100 scroll-mt-20">HTTP Headers</h2>

        <div className="mb-6">
          <h3 className="text-lg font-semibold mb-3 text-slate-900 dark:text-slate-100">Response Headers (402)</h3>
          <div className="bg-slate-50 dark:bg-slate-900 rounded-lg p-4 mb-4">
            <table className="w-full text-sm">
              <thead>
                <tr className="border-b border-slate-200 dark:border-slate-700">
                  <th className="text-left py-2 text-slate-900 dark:text-slate-100">Header</th>
                  <th className="text-left py-2 text-slate-900 dark:text-slate-100">Description</th>
                </tr>
              </thead>
              <tbody className="text-slate-700 dark:text-slate-300">
                <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">WWW-Authenticate</code></td><td>Challenge (base64)</td></tr>
                <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">Payment-Required</code></td><td>Always "402"</td></tr>
                <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">Cache-Control</code></td><td>"no-store" (prevent caching)</td></tr>
              </tbody>
            </table>
          </div>
        </div>

        <div className="mb-6">
          <h3 className="text-lg font-semibold mb-3 text-slate-900 dark:text-slate-100">Request Headers (Payment)</h3>
          <div className="bg-slate-50 dark:bg-slate-900 rounded-lg p-4 mb-4">
            <table className="w-full text-sm">
              <thead>
                <tr className="border-b border-slate-200 dark:border-slate-700">
                  <th className="text-left py-2 text-slate-900 dark:text-slate-100">Header</th>
                  <th className="text-left py-2 text-slate-900 dark:text-slate-100">Description</th>
                </tr>
              </thead>
              <tbody className="text-slate-700 dark:text-slate-300">
                <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">Authorization</code></td><td>Credential (base64)</td></tr>
                <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">Content-Digest</code></td><td>Body digest (if binding)</td></tr>
              </tbody>
            </table>
          </div>
        </div>

        <div className="mb-6">
          <h3 className="text-lg font-semibold mb-3 text-slate-900 dark:text-slate-100">Response Headers (200)</h3>
          <div className="bg-slate-50 dark:bg-slate-900 rounded-lg p-4 mb-4">
            <table className="w-full text-sm">
              <thead>
                <tr className="border-b border-slate-200 dark:border-slate-700">
                  <th className="text-left py-2 text-slate-900 dark:text-slate-100">Header</th>
                  <th className="text-left py-2 text-slate-900 dark:text-slate-100">Description</th>
                </tr>
              </thead>
              <tbody className="text-slate-700 dark:text-slate-300">
                <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">Payment-Receipt</code></td><td>Receipt (base64)</td></tr>
              </tbody>
            </table>
          </div>
        </div>

        <h2 id="verification" className="text-2xl font-bold mb-4 text-slate-900 dark:text-slate-100 scroll-mt-20">Verification</h2>

        <div className="mb-6">
          <h3 className="text-lg font-semibold mb-3 text-slate-900 dark:text-slate-100">Verifier Trait</h3>
          <p className="mb-4 text-slate-700 dark:text-slate-300">
            Implement custom verification logic via the <code className="bg-slate-100 dark:bg-slate-800 px-2 py-0.5 rounded font-mono text-sm">Verifier</code> trait:
          </p>
          <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4 mb-4">
            <pre className="text-sm" style={{color: '#00C08B'}}>
              <code>{`use mpp_near::Verifier;

pub trait Verifier {
    fn verify(&self, credential: &Credential)
        -> VerificationResult;
}

// Implementation provided for:
// - Challenge binding verification
// - Signature validation
// - Expiry checking`}</code>
            </pre>
          </div>
        </div>

        <div className="mb-6">
          <h3 className="text-lg font-semibold mb-3 text-slate-900 dark:text-slate-100">VerificationResult</h3>
          <div className="bg-slate-50 dark:bg-slate-900 rounded-lg p-4">
            <pre className="text-sm text-slate-700 dark:text-slate-300"><code>{`pub enum VerificationResult {
    Valid,                      // Payment verified
    Invalid(String),            // Verification failed with reason
    Expired,                    // Challenge/expired
}`}</code></pre>
          </div>
        </div>

        <h2 id="custom-methods" className="text-2xl font-bold mb-4 text-slate-900 dark:text-slate-100 scroll-mt-20">Custom Payment Methods</h2>

        <div className="mb-6">
          <p className="mb-4 text-slate-700 dark:text-slate-300">
            Implement custom payment methods via the <code className="bg-slate-100 dark:bg-slate-800 px-2 py-0.5 rounded font-mono text-sm">Method</code> trait. This allows integration with any payment network.
          </p>

          <h3 className="text-lg font-semibold mb-3 text-slate-900 dark:text-slate-100">Method Trait</h3>
          <div className="bg-slate-50 dark:bg-slate-900 rounded-lg p-4 mb-4 overflow-x-auto">
            <pre className="text-sm text-slate-700 dark:text-slate-300"><code>{`use mpp_near::Method;
use async_trait::async_trait;

#[async_trait]
pub trait Method: Send + Sync {
    /// Method identifier (e.g., "near-intents", "stripe")
    fn id(&self) -> &str;

    /// Build a challenge for this method
    fn build_challenge(
        &self,
        request: &PaymentRequest,
        secret: &[u8]
    ) -> Result<Challenge>;

    /// Verify a payment proof
    async fn verify(
        &self,
        request: &PaymentRequest,
        proof: &PaymentProof,
    ) -> Result<bool>;

    /// Extract request from challenge
    fn extract_request(
        &self,
        challenge: &Challenge
    ) -> Result<PaymentRequest>;

    /// Extract proof from credential
    fn extract_proof(
        &self,
        credential: &Credential
    ) -> Result<PaymentProof>;

    /// Verify full credential
    async fn verify_credential(
        &self,
        challenge: &Challenge,
        credential: &Credential,
    ) -> Result<bool>;
}`}</code></pre>
          </div>

          <h4 className="text-md font-semibold mb-2 text-slate-900 dark:text-slate-100 mt-6">PaymentRequest</h4>
          <div className="bg-slate-50 dark:bg-slate-900 rounded-lg p-4 mb-4 overflow-x-auto">
            <pre className="text-sm text-slate-700 dark:text-slate-300"><code>{`pub struct PaymentRequest {
    pub amount: String,           // Amount to pay
    pub currency: Option<String>,  // Currency/token
    pub token_id: Option<String>,  // Token contract
    pub recipient: String,         // Recipient address
    pub chain: Option<String>,     // Blockchain/network
    pub challenge_id: String,      // Challenge reference
    pub realm: String,             // Protection space
    pub method: String,            // Payment method
    pub intent: String,            // Payment intent
}`}</code></pre>
          </div>

          <h4 className="text-md font-semibold mb-2 text-slate-900 dark:text-slate-100 mt-6">PaymentProof</h4>
          <div className="bg-slate-50 dark:bg-slate-900 rounded-lg p-4 mb-4 overflow-x-auto">
            <pre className="text-sm text-slate-700 dark:text-slate-300"><code>{`pub struct PaymentProof {
    pub proof: String,                        // Transaction hash
    pub account: Option<String>,               // Payer account
    pub signature: Option<String>,             // Transaction signature
    pub public_key: Option<String>,            // Payer public key
    pub extra: HashMap<String, Value>,         // Additional fields
}`}</code></pre>
          </div>
        </div>

        <div className="mb-6">
          <h3 className="text-lg font-semibold mb-3 text-slate-900 dark:text-slate-100">MethodRegistry</h3>
          <p className="mb-4 text-slate-700 dark:text-slate-300">
            Register and manage multiple payment methods:
          </p>
          <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4 mb-4">
            <pre className="text-sm" style={{color: '#00C08B'}}>
              <code>{`use mpp_near::MethodRegistry;

let mut registry = MethodRegistry::new();
registry.register(MyCustomMethod);
registry.register(AnotherMethod);

// Check if method exists
if registry.contains("my-method") {
    let method = registry.get("my-method").unwrap();
    // Use method to verify payment
}

// List all methods
let methods = registry.list();`}</code>
            </pre>
          </div>

          <h4 className="text-md font-semibold mb-2 text-slate-900 dark:text-slate-100 mt-6">Registry Methods</h4>
          <div className="bg-slate-50 dark:bg-slate-900 rounded-lg p-4">
            <table className="w-full text-sm">
              <thead>
                <tr className="border-b border-slate-200 dark:border-slate-700">
                  <th className="text-left py-2 text-slate-900 dark:text-slate-100">Method</th>
                  <th className="text-left py-2 text-slate-900 dark:text-slate-100">Description</th>
                </tr>
              </thead>
              <tbody className="text-slate-700 dark:text-slate-300">
                <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">new()</code></td><td>Create empty registry</td></tr>
                <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">register()</code></td><td>Register a method</td></tr>
                <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">get()</code></td><td>Get method by ID</td></tr>
                <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">contains()</code></td><td>Check if method exists</td></tr>
                <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">list()</code></td><td>List all method IDs</td></tr>
              </tbody>
            </table>
          </div>
        </div>

        <h2 id="errors" className="text-2xl font-bold mb-4 text-slate-900 dark:text-slate-100 scroll-mt-20">Error Handling</h2>

        <div className="mb-6">
          <h3 className="text-lg font-semibold mb-3 text-slate-900 dark:text-slate-100">Problem Type (RFC 9457)</h3>
          <p className="mb-4 text-slate-700 dark:text-slate-300">
            Return structured error responses with <code className="bg-slate-100 dark:bg-slate-800 px-2 py-0.5 rounded font-mono text-sm">Problem</code>:
          </p>
          <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4 mb-4">
            <pre className="text-sm" style={{color: '#00C08B'}}>
              <code>{`use mpp_near::Problem;

// Payment insufficient
let problem = Problem::payment_insufficient(
    "0.01",    // required
    "0.001",   // actual
);

// Verification failed
let problem = Problem::verification_failed(
    "Invalid signature"
);

// Invalid challenge
let problem = Problem::invalid_challenge(
    "Challenge expired"
);

// Returns JSON:
// {
//   "type": "https://mpp.dev/problems/payment-insufficient",
//   "title": "Payment Insufficient",
//   "detail": "Required: 0.01, Actual: 0.001",
//   "status": 402
// }
let json = serde_json::to_string(&problem)?;`}</code>
            </pre>
          </div>

          <h4 className="text-md font-semibold mb-2 text-slate-900 dark:text-slate-100 mt-6">Error Types (Problem)</h4>
          <div className="bg-slate-50 dark:bg-slate-900 rounded-lg p-4 mb-4">
            <table className="w-full text-sm">
              <thead>
                <tr className="border-b border-slate-200 dark:border-slate-700">
                  <th className="text-left py-2 text-slate-900 dark:text-slate-100">Type</th>
                  <th className="text-left py-2 text-slate-900 dark:text-slate-100">Description</th>
                </tr>
              </thead>
              <tbody className="text-slate-700 dark:text-slate-300">
                <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">payment_insufficient</code></td><td>Amount too low</td></tr>
                <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">verification_failed</code></td><td>Invalid proof/signature</td></tr>
                <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">invalid_challenge</code></td><td>Challenge malformed</td></tr>
                <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">challenge_expired</code></td><td>Challenge too old</td></tr>
                <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">unsupported_method</code></td><td>Unknown payment method</td></tr>
              </tbody>
            </table>
          </div>

          <h4 className="text-md font-semibold mb-2 text-slate-900 dark:text-slate-100 mt-6">Error Enum</h4>
          <p className="mb-4 text-slate-700 dark:text-slate-300">
            Full error enum with all variants:
          </p>
          <div className="bg-slate-50 dark:bg-slate-900 rounded-lg p-4 mb-4 overflow-x-auto">
            <pre className="text-sm text-slate-700 dark:text-slate-300"><code>{`pub enum Error {
    InvalidChallenge(String),      // Invalid challenge format
    InvalidCredential(String),      // Invalid credential format
    ChallengeExpired,               // Challenge expired
    ChallengeNotFound,              // Challenge not found
    VerificationFailed(String),     // Payment verification failed
    UnsupportedMethod(String),      // Unsupported payment method
    InvalidAmount(String),          // Invalid amount
    Http(http::Error),              // HTTP error
    Json(serde_json::Error),        // JSON error
    Base64(base64::DecodeError),    // Base64 decode error
    Other(String),                  // Other error
}`}</code></pre>
          </div>

          <h4 className="text-md font-semibold mb-2 text-slate-900 dark:text-slate-100 mt-6">Result Type</h4>
          <p className="mb-4 text-slate-700 dark:text-slate-300">
            Type alias for Result with Error:
          </p>
          <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4 mb-4">
            <pre className="text-sm" style={{color: '#00C08B'}}>
              <code>{`use mpp_near::Result;

// Result type alias
pub type Result<T> = std::result::Result<T, Error>;

// Usage
fn create_challenge() -> Result<Challenge> {
    // Returns Result<Challenge, Error>
    Ok(challenge)
}`}</code>
            </pre>
          </div>
        </div>

        <h2 id="constants" className="text-2xl font-bold mb-4 text-slate-900 dark:text-slate-100 scroll-mt-20">Constants</h2>

        <div className="mb-6">
          <p className="mb-4 text-slate-700 dark:text-slate-300">
            MPP-NEAR provides protocol-level constants for versioning and defaults:
          </p>

          <div className="bg-slate-50 dark:bg-slate-900 rounded-lg p-4 mb-4 overflow-x-auto">
            <pre className="text-sm text-slate-700 dark:text-slate-300"><code>{`/// MPP protocol version
pub const VERSION: &str = "MPP/1.0";

/// Default challenge TTL in seconds
pub const DEFAULT_CHALLENGE_TTL: i64 = 300; // 5 minutes`}</code></pre>
          </div>

          <h4 className="text-md font-semibold mb-2 text-slate-900 dark:text-slate-100 mt-6">Usage</h4>
          <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4 mb-4">
            <pre className="text-sm" style={{color: '#00C08B'}}>
              <code>{`use mpp_near::{VERSION, DEFAULT_CHALLENGE_TTL};

// Use in responses
response.headers.insert("MPP-Version", VERSION.parse()?);

// Use for default TTL
let ttl = DEFAULT_CHALLENGE_TTL;  // 300 seconds

// Override as needed
let challenge = Challenge::builder()
    .expires(DEFAULT_CHALLENGE_TTL)
    .build()?;`}</code>
            </pre>
          </div>

          <div className="bg-blue-50 dark:bg-blue-500/10 border border-blue-200 dark:border-blue-500/20 rounded-lg p-4">
            <h4 className="font-semibold text-blue-900 dark:text-blue-100 mb-2">Note</h4>
            <p className="text-sm text-blue-800 dark:text-blue-200">
              <code className="bg-blue-100 dark:bg-blue-900 px-1 rounded">VERSION</code> is included in all challenges and credentials for protocol negotiation. <code className="bg-blue-100 dark:bg-blue-900 px-1 rounded">DEFAULT_CHALLENGE_TTL</code> is used when no explicit TTL is set via <code className="bg-blue-100 dark:bg-blue-900 px-1 rounded">.expires()</code>.
            </p>
          </div>
        </div>

        <h2 id="examples" className="text-2xl font-bold mb-4 text-slate-900 dark:text-slate-100 scroll-mt-20">Complete Example</h2>

        <div className="mb-6">
          <h3 className="text-lg font-semibold mb-3 text-slate-900 dark:text-slate-100">Payment-Gated API Endpoint</h3>
          <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4 mb-4">
            <pre className="text-sm" style={{color: '#00C08B'}}>
              <code>{`use axum::{Json, http::StatusCode, extract::State};
use mpp_near::{Challenge, Credential, RequestData, Receipt};
use std::sync::Arc;

async fn protected_image_generation(
    State(secret): State<Arc<Vec<u8>>>,
) -> Result<Json<&'static str>, StatusCode> {
    // Create payment challenge
    let challenge = Challenge::builder()
        .realm("api.example.com")
        .method("near-intents")
        .intent("charge")
        .request(
            RequestData::new("0.001", "wallet.near")
                .currency("USDC")
        )
        .description("AI image generation")
        .secret(&secret)
        .build()?;

    // Return 402 with challenge
    Err(StatusCode::PAYMENT_REQUIRED)
        // Include WWW-Authenticate header with challenge
}`}</code>
            </pre>
          </div>
        </div>

        <h2 id="middleware" className="text-2xl font-bold mb-4 text-slate-900 dark:text-slate-100 scroll-mt-20">Axum Middleware</h2>

        <p className="mb-4 text-slate-700 dark:text-slate-300">
          Pre-built Axum middleware and extractors are available:
        </p>

        <div className="bg-slate-50 dark:bg-slate-900 rounded-lg p-4 mb-4">
          <pre className="text-sm text-slate-700 dark:text-slate-300"><code>{`// Middleware features:
- Payment verification middleware
- Credential extractor
- Challenge builder helper
- Problem response helper

// Available with:
mpp-near = { git = "https://github.com/kampouse/mpp-near",
            features = ["server"] }`}</code></pre>
        </div>

        <div className="mt-8 p-6 bg-slate-50 dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg">
          <h4 className="font-semibold text-slate-900 dark:text-slate-100 mb-2">See Also</h4>
          <ul className="space-y-2 text-sm">
            <li><a href="/docs/quickstart" className="hover:underline" style={{color: '#00C08B'}}>Quick Start →</a></li>
            <li><a href="https://docs.rs/mpp-near" target="_blank" rel="noopener noreferrer" className="hover:underline" style={{color: '#00C08B'}}>Rust Docs →</a></li>
            <li><a href="https://github.com/kampouse/mpp-near" target="_blank" rel="noopener noreferrer" className="hover:underline" style={{color: '#00C08B'}}>Source Code →</a></li>
          </ul>
        </div>
      </div>

      {/* Table of Contents Sidebar */}
      <aside className="hidden lg:block w-56 flex-shrink-0">
        <div className="sticky top-8">
          <h4 className="text-xs font-semibold text-slate-500 uppercase tracking-wider mb-4">
            On this page
          </h4>
          <nav>
            <ul className="space-y-3">
              {sections.map((section) => (
                <li key={section.id}>
                  <a
                    href={`#${section.id}`}
                    className="flex items-center gap-2 text-sm text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-100 transition-colors"
                  >
                    <span className="text-base opacity-70">{section.icon}</span>
                    <span>{section.title}</span>
                  </a>
                </li>
              ))}
            </ul>
          </nav>
        </div>
      </aside>
    </div>
  )
}
