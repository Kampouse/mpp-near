export default function QuickstartPage() {
  return (
    <div>
      <h1 className="text-4xl font-bold mb-4 text-slate-900 dark:text-slate-50">Quick Start</h1>
      <p className="text-xl text-slate-700 dark:text-slate-300 mb-12">
        Get started with MPP-NEAR in 5 minutes. Choose your path below.
      </p>

      {/* Service Provider Section */}
      <section className="mb-16 pb-12 border-b border-slate-200 dark:border-slate-800">
        <div className="flex items-center gap-3 mb-6">
          <div className="w-12 h-12 bg-emerald-50 dark:bg-emerald-500/10 rounded-lg flex items-center justify-center">
            <span className="text-2xl">🖥️</span>
          </div>
          <div>
            <h2 className="text-2xl font-bold text-slate-900 dark:text-slate-100">For Service Providers</h2>
            <p className="text-sm text-slate-600 dark:text-slate-400">Build payment-gated APIs</p>
          </div>
        </div>

        <p className="text-slate-700 dark:text-slate-300 mb-6">
          Use the Rust SDK to add HTTP 402 payments to your APIs. Monetize per-request with type-safe primitives.
        </p>

        <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4 mb-6">
          <pre className="text-sm text-emerald-400">
            <code>{`# Add to Cargo.toml
[dependencies]
mpp-near = { git = "https://github.com/kampouse/mpp-near", features = ["server"] }`}</code>
          </pre>
        </div>

        <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4 mb-6">
          <pre className="text-sm text-emerald-400">
            <code>{`// Create a payment challenge
use mpp_near::{Challenge, RequestData};

let challenge = Challenge::builder()
    .realm("api.example.com")
    .method("near-intents")
    .request(RequestData::new("0.001", "wallet.near"))
    .secret(b"your-hmac-secret")
    .build()?;`}</code>
          </pre>
        </div>

        <a
          href="/docs/api/overview"
          className="inline-flex items-center gap-2 text-emerald-600 dark:text-emerald-400 hover:text-emerald-700 dark:hover:text-emerald-300 font-medium"
        >
          View API Documentation →
        </a>
      </section>

      {/* Agent & Client Section */}
      <section className="mb-16 pb-12 border-b border-slate-200 dark:border-slate-800">
        <div className="flex items-center gap-3 mb-6">
          <div className="w-12 h-12 bg-violet-50 dark:bg-violet-500/10 rounded-lg flex items-center justify-center">
            <span className="text-2xl">🤖</span>
          </div>
          <div>
            <h2 className="text-2xl font-bold text-slate-900 dark:text-slate-100">For Agents & Clients</h2>
            <p className="text-sm text-slate-600 dark:text-slate-400">Make gasless payments</p>
          </div>
        </div>

        <p className="text-slate-700 dark:text-slate-300 mb-6">
          Use the CLI tool to make gasless payments, swap tokens, and transact agent-to-agent.
        </p>

        <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4 mb-6">
          <pre className="text-sm text-emerald-400">
            <code>{`# Clone and install
git clone https://github.com/kampouse/mpp-near.git
cd mpp-near
cargo install --path . --features intents`}</code>
          </pre>
        </div>

        <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4 mb-6">
          <pre className="text-sm text-emerald-400">
            <code>{`# Register for gasless wallet
mpp-near register

# Fund your wallet
mpp-near fund-link --amount 0.1 --token near

# Make a gasless payment
mpp-near pay --recipient merchant.near --amount 10 --token usdc`}</code>
          </pre>
        </div>

        <a
          href="/docs/cli/overview"
          className="inline-flex items-center gap-2 text-violet-600 dark:text-violet-400 hover:text-violet-700 dark:hover:text-violet-300 font-medium"
        >
          View CLI Documentation →
        </a>
      </section>

      {/* Next Steps */}
      <section>
        <h2 className="text-2xl font-bold text-slate-900 dark:text-slate-100 mb-6">Next Steps</h2>
        <div className="grid md:grid-cols-3 gap-4">
          <a
            href="/docs/api/overview"
            className="block p-4 rounded-lg border border-slate-200 dark:border-slate-800 hover:border-emerald-500 dark:hover:border-emerald-500 transition-colors"
          >
            <h3 className="font-semibold text-slate-900 dark:text-slate-100 mb-1">Rust SDK</h3>
            <p className="text-sm text-slate-700 dark:text-slate-300">API documentation</p>
          </a>
          <a
            href="/docs/cli/overview"
            className="block p-4 rounded-lg border border-slate-200 dark:border-slate-800 hover:border-violet-500 dark:hover:border-violet-500 transition-colors"
          >
            <h3 className="font-semibold text-slate-900 dark:text-slate-100 mb-1">CLI Reference</h3>
            <p className="text-sm text-slate-700 dark:text-slate-300">Command reference</p>
          </a>
          <a
            href="/docs/outlayer"
            className="block p-4 rounded-lg border border-slate-200 dark:border-slate-800 hover:border-amber-500 dark:hover:border-amber-500 transition-colors"
          >
            <h3 className="font-semibold text-slate-900 dark:text-slate-100 mb-1">OutLayer</h3>
            <p className="text-sm text-slate-700 dark:text-slate-300">Gasless integration</p>
          </a>
        </div>
      </section>
    </div>
  )
}
