export default function InstallationPage() {
  return (
    <div>
      <h1 className="text-4xl font-bold mb-4 text-slate-900 dark:text-slate-50">Installation</h1>
      <p className="text-xl text-slate-700 dark:text-slate-300 mb-8">
        Install MPP-NEAR to start building payment-gated APIs or empowering agents with gasless payments.
      </p>

      <div className="bg-amber-50 dark:bg-amber-500/10 border border-amber-200 dark:border-amber-500/20 rounded-lg p-4 mb-8">
        <p className="text-amber-900 dark:text-amber-100 text-sm">
          <strong>Note:</strong> MPP-NEAR is not yet available on crates.io. Please install from source.
        </p>
      </div>

      <h2 className="text-2xl font-bold mb-4 text-slate-900 dark:text-slate-100">Build Requirements</h2>
      <ul className="list-disc pl-6 mb-8 space-y-2 text-slate-700 dark:text-slate-300">
        <li><strong className="text-slate-900 dark:text-slate-100">Rust 1.70</strong> or later</li>
        <li><strong className="text-slate-900 dark:text-slate-100">Cargo</strong> (comes with Rust)</li>
      </ul>

      <h2 className="text-2xl font-bold mb-4 text-slate-900 dark:text-slate-100">Option 1: Install CLI Tool</h2>
      <p className="mb-4 text-slate-700 dark:text-slate-300">For agents, clients, and command-line usage:</p>

      <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4 mb-8">
        <pre className="text-sm text-emerald-400">
          <code>{`# Clone the repository
git clone https://github.com/kampouse/mpp-near.git
cd mpp-near

# Install with intents feature
cargo install --path . --features intents`}</code>
        </pre>
      </div>

      <h2 className="text-2xl font-bold mb-4 text-slate-900 dark:text-slate-100">Option 2: Add as Library</h2>
      <p className="mb-4 text-slate-700 dark:text-slate-300">For building payment-gated APIs in Rust:</p>

      <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4 mb-8">
        <pre className="text-sm text-emerald-400">
          <code>{`# Cargo.toml
[dependencies]
mpp-near = { git = "https://github.com/kampouse/mpp-near" }`}</code>
        </pre>
      </div>

      <h2 className="text-2xl font-bold mb-4 text-slate-900 dark:text-slate-100">Verify Installation</h2>

      <h3 className="text-xl font-semibold mb-2 text-slate-900 dark:text-slate-100">CLI Tool</h3>
      <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4 mb-8">
        <pre className="text-sm text-emerald-400">
          <code>{`mpp-near --version`}</code>
        </pre>
      </div>

      <h2 className="text-2xl font-bold mb-4 text-slate-900 dark:text-slate-100">Next Steps</h2>
      <ul className="list-disc pl-6 space-y-2 text-slate-700 dark:text-slate-300">
        <li><a href="/docs/quickstart" className="text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300">Quick Start Guide</a></li>
        <li><a href="/docs/cli/overview" className="text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300">CLI Documentation</a></li>
        <li><a href="/docs/api/overview" className="text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300">API Documentation</a></li>
      </ul>
    </div>
  )
}
