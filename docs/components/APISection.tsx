export default function APISection() {
  return (
    <section id="api" className="py-20 px-6 bg-slate-50 dark:bg-slate-900/50">
      <div className="max-w-6xl mx-auto">
        <div className="text-center mb-16">
          <div className="inline-flex items-center gap-2 px-4 py-2 bg-emerald-50 dark:bg-emerald-500/10 border border-emerald-200 dark:border-emerald-500/20 rounded-full text-emerald-600 dark:text-emerald-400 text-sm mb-6">
            <span className="text-lg">🖥️</span>
            For Service Providers
          </div>
          <h2 className="text-4xl md:text-5xl font-bold mb-4 text-slate-900 dark:text-slate-50">
            Rust API for Services
          </h2>
          <p className="text-xl text-slate-600 dark:text-slate-400 max-w-3xl mx-auto">
            Build payment-gated APIs and monetize your services with type-safe Rust primitives.
            Accept NEAR payments for your API endpoints automatically.
          </p>
        </div>

        <div className="bg-white dark:bg-slate-800/50 border border-slate-200 dark:border-slate-700 rounded-xl p-6 mb-12 shadow-sm dark:shadow-none">
          <div className="flex items-start gap-4">
            <div className="text-3xl">🎯</div>
            <div>
              <h4 className="font-semibold text-emerald-600 dark:text-emerald-400 mb-2">Perfect For:</h4>
              <div className="grid md:grid-cols-2 gap-4 text-sm text-slate-600 dark:text-slate-300">
                <div className="flex items-center gap-2">
                  <span className="text-emerald-600 dark:text-emerald-400">✓</span>
                  <span>API monetization</span>
                </div>
                <div className="flex items-center gap-2">
                  <span className="text-emerald-600 dark:text-emerald-400">✓</span>
                  <span>SaaS platforms</span>
                </div>
                <div className="flex items-center gap-2">
                  <span className="text-emerald-600 dark:text-emerald-400">✓</span>
                  <span>Data APIs</span>
                </div>
                <div className="flex items-center gap-2">
                  <span className="text-emerald-600 dark:text-emerald-400">✓</span>
                  <span>AI/ML APIs</span>
                </div>
                <div className="flex items-center gap-2">
                  <span className="text-emerald-600 dark:text-emerald-400">✓</span>
                  <span>Compute services</span>
                </div>
                <div className="flex items-center gap-2">
                  <span className="text-emerald-600 dark:text-emerald-400">✓</span>
                  <span>Content platforms</span>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div className="grid md:grid-cols-2 gap-8 mb-12">
          <div className="bg-white dark:bg-slate-800/50 border border-slate-200 dark:border-slate-700 rounded-xl p-8 shadow-sm dark:shadow-none">
            <h3 className="text-2xl font-bold mb-4 text-emerald-600 dark:text-emerald-400">Create Payment Challenges</h3>
            <p className="text-slate-600 dark:text-slate-300 mb-6">
              Generate HTTP 402 payment challenges when clients access your paid endpoints.
            </p>
            <pre className="bg-slate-100 dark:bg-slate-900 rounded-lg p-4 text-sm overflow-x-auto">
              <code className="text-emerald-600 dark:text-emerald-400">{`use mpp_near::{Challenge, RequestData};

let challenge = Challenge::builder()
    .realm("api.example.com")
    .method("near-intents")
    .intent("charge")
    .request(RequestData::new("0.001", "wallet.near"))
    .secret(b"hmac-secret")
    .build()?;`}</code>
            </pre>
          </div>

          <div className="bg-white dark:bg-slate-800/50 border border-slate-200 dark:border-slate-700 rounded-xl p-8 shadow-sm dark:shadow-none">
            <h3 className="text-2xl font-bold mb-4 text-violet-600 dark:text-violet-400">Verify Payments Securely</h3>
            <p className="text-slate-600 dark:text-slate-300 mb-6">
              Verify payment credentials with HMAC binding and challenge echo verification.
            </p>
            <pre className="bg-slate-100 dark:bg-slate-900 rounded-lg p-4 text-sm overflow-x-auto">
              <code className="text-emerald-600 dark:text-emerald-400">{`use mpp_near::Credential;

let credential = Credential::from_authorization(auth)?;

if credential.verify_challenge_echo(&challenge)
    && challenge.verify_binding(b"secret") {
    // Payment valid - return requested data
}`}</code>
            </pre>
          </div>
        </div>

        <div className="grid md:grid-cols-3 gap-6">
          <div className="bg-white dark:bg-slate-800/30 border border-slate-200 dark:border-slate-700 rounded-lg p-6 hover:border-emerald-500/30 dark:hover:border-emerald-500/20 transition-colors shadow-sm dark:shadow-none">
            <div className="w-12 h-12 bg-emerald-50 dark:bg-emerald-500/10 rounded-lg flex items-center justify-center mb-4">
              <span className="text-2xl">🔒</span>
            </div>
            <h4 className="text-lg font-bold mb-2 text-slate-900 dark:text-slate-100">Stateless Verification</h4>
            <p className="text-slate-600 dark:text-slate-400 text-sm">
              HMAC binding lets you verify payments without database storage. Scale horizontally without persistence.
            </p>
          </div>

          <div className="bg-white dark:bg-slate-800/30 border border-slate-200 dark:border-slate-700 rounded-lg p-6 hover:border-violet-500/30 dark:hover:border-violet-500/20 transition-colors shadow-sm dark:shadow-none">
            <div className="w-12 h-12 bg-violet-50 dark:bg-violet-500/10 rounded-lg flex items-center justify-center mb-4">
              <span className="text-2xl">⚡</span>
            </div>
            <h4 className="text-lg font-bold mb-2 text-slate-900 dark:text-slate-100">Type-Safe API</h4>
            <p className="text-slate-600 dark:text-slate-400 text-sm">
              Strongly typed builders catch errors at compile time, not runtime. Built for production services.
            </p>
          </div>

          <div className="bg-white dark:bg-slate-800/30 border border-slate-200 dark:border-slate-700 rounded-lg p-6 hover:border-teal-500/30 dark:hover:border-teal-500/20 transition-colors shadow-sm dark:shadow-none">
            <div className="w-12 h-12 bg-teal-50 dark:bg-teal-500/10 rounded-lg flex items-center justify-center mb-4">
              <span className="text-2xl">📋</span>
            </div>
            <h4 className="text-lg font-bold mb-2 text-slate-900 dark:text-slate-100">Standards Compliant</h4>
            <p className="text-slate-600 dark:text-slate-400 text-sm">
              Implements MPP-1.0, RFC 9457, and RFC 9530 standards. Works with any MPP client.
            </p>
          </div>
        </div>

        <div className="mt-12 text-center">
          <a
            href="/docs"
            className="inline-flex items-center gap-2 px-8 py-3 bg-slate-900 dark:bg-slate-50 hover:bg-slate-800 dark:hover:bg-slate-100 rounded-lg font-medium transition-all duration-200 text-white dark:text-slate-900 shadow-lg shadow-slate-900/10 dark:shadow-none"
          >
            Build Payment-Gated APIs
            <span>→</span>
          </a>
        </div>
      </div>
    </section>
  )
}
