export default function CLISection() {
  return (
    <section id="cli" className="py-20 px-6 bg-slate-50 dark:bg-slate-900/50">
      <div className="max-w-6xl mx-auto">
        <div className="text-center mb-16">
          <div className="inline-flex items-center gap-2 px-4 py-2 bg-violet-50 dark:bg-violet-500/10 border border-violet-200 dark:border-violet-500/20 rounded-full text-violet-600 dark:text-violet-400 text-sm mb-6">
            <span className="text-lg">🤖</span>
            For Agents & Clients
          </div>
          <h2 className="text-4xl md:text-5xl font-bold mb-4 text-slate-900 dark:text-slate-50">
            CLI for Autonomous Agents
          </h2>
          <p className="text-xl text-slate-600 dark:text-slate-400 max-w-3xl mx-auto">
            Command-line interface for AI agents, scripts, and automated systems to make payments,
            swap tokens, and interact with paid APIs automatically.
          </p>
        </div>

        <div className="bg-white dark:bg-slate-800/50 border border-slate-200 dark:border-slate-700 rounded-xl p-6 mb-12 shadow-sm dark:shadow-none">
          <div className="flex items-start gap-4">
            <div className="text-3xl">🤖</div>
            <div>
              <h4 className="font-semibold text-violet-600 dark:text-violet-400 mb-2">Perfect For:</h4>
              <div className="grid md:grid-cols-2 gap-4 text-sm text-slate-600 dark:text-slate-300">
                <div className="flex items-center gap-2">
                  <span className="text-emerald-600 dark:text-emerald-400">✓</span>
                  <span>AI agents paying for services</span>
                </div>
                <div className="flex items-center gap-2">
                  <span className="text-emerald-600 dark:text-emerald-400">✓</span>
                  <span>Automated scripts</span>
                </div>
                <div className="flex items-center gap-2">
                  <span className="text-emerald-600 dark:text-emerald-400">✓</span>
                  <span>Cron jobs & schedulers</span>
                </div>
                <div className="flex items-center gap-2">
                  <span className="text-emerald-600 dark:text-emerald-400">✓</span>
                  <span>IoT devices</span>
                </div>
                <div className="flex items-center gap-2">
                  <span className="text-emerald-600 dark:text-emerald-400">✓</span>
                  <span>Agent-to-agent payments</span>
                </div>
                <div className="flex items-center gap-2">
                  <span className="text-emerald-600 dark:text-emerald-400">✓</span>
                  <span>Batch operations</span>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div className="grid md:grid-cols-2 gap-8 mb-12">
          <div className="bg-white dark:bg-slate-800/50 border border-slate-200 dark:border-slate-700 rounded-xl p-8 shadow-sm dark:shadow-none">
            <h3 className="text-2xl font-bold mb-4 text-violet-600 dark:text-violet-400">Gasless Payments</h3>
            <p className="text-slate-600 dark:text-slate-300 mb-6">
              Agents can pay for API access without managing gas. Powered by <a href="https://outlayer.fastnear.com" target="_blank" rel="noopener noreferrer" className="text-violet-600 dark:text-violet-400 hover:underline">OutLayer</a> custody wallets via NEAR Intents.
            </p>
            <pre className="bg-slate-100 dark:bg-slate-900 rounded-lg p-4 text-sm overflow-x-auto">
              <code className="text-violet-600 dark:text-violet-400">{`# Agent pays for API access
mpp-near pay \\
  --recipient api-provider.near \\
  --amount 0.001 \\
  --memo "API access - image generation"

# Pay with USDC (gasless!)
mpp-near pay \\
  --recipient merchant.near \\
  --amount 10 \\
  --token usdc`}</code>
            </pre>
          </div>

          <div className="bg-white dark:bg-slate-800/50 border border-slate-200 dark:border-slate-700 rounded-xl p-8 shadow-sm dark:shadow-none">
            <h3 className="text-2xl font-bold mb-4 text-rose-600 dark:text-rose-400">Agent-to-Agent Payments</h3>
            <p className="text-slate-600 dark:text-slate-300 mb-6">
              Create payment checks that other agents can claim. Perfect for agent commerce.
            </p>
            <pre className="bg-slate-100 dark:bg-slate-900 rounded-lg p-4 text-sm overflow-x-auto">
              <code className="text-rose-600 dark:text-rose-400">{`# Agent A creates payment check
mpp-near create-check \\
  --amount 50 \\
  --token usdc \\
  --memo "Data processing services" \\
  --expires-in 86400

# Agent B claims the payment
mpp-near claim-check \\
  --check-key <check_key_from_agent_a>`}</code>
            </pre>
          </div>
        </div>

        <div className="grid md:grid-cols-2 gap-8">
          <div className="bg-white dark:bg-slate-800/30 border border-slate-200 dark:border-slate-700 rounded-lg p-6 hover:border-amber-500/30 dark:hover:border-amber-500/20 transition-colors shadow-sm dark:shadow-none">
            <h4 className="text-lg font-bold mb-3 text-amber-600 dark:text-amber-400">Token Swaps</h4>
            <p className="text-slate-600 dark:text-slate-400 text-sm mb-4">
              Agents can swap tokens across 20+ blockchains gaslessly
            </p>
            <pre className="bg-slate-100 dark:bg-slate-900 rounded-lg p-3 text-xs">
              <code className="text-amber-600 dark:text-amber-400">{`# Swap NEAR to USDC for payments
mpp-near swap --from near --to usdc --amount 1`}</code>
            </pre>
          </div>

          <div className="bg-white dark:bg-slate-800/30 border border-slate-200 dark:border-slate-700 rounded-lg p-6 hover:border-violet-500/30 dark:hover:border-violet-500/20 transition-colors shadow-sm dark:shadow-none">
            <h4 className="text-lg font-bold mb-3 text-violet-600 dark:text-violet-400">Balance Management</h4>
            <p className="text-slate-600 dark:text-slate-400 text-sm mb-4">
              Monitor agent wallet balances automatically
            </p>
            <pre className="bg-slate-100 dark:bg-slate-900 rounded-lg p-3 text-xs">
              <code className="text-violet-600 dark:text-violet-400">{`# Check agent balance
mpp-near balance --api-key wk_...`}</code>
            </pre>
          </div>

          <div className="bg-white dark:bg-slate-800/30 border border-slate-200 dark:border-slate-700 rounded-lg p-6 hover:border-rose-500/30 dark:hover:border-rose-500/20 transition-colors shadow-sm dark:shadow-none">
            <h4 className="text-lg font-bold mb-3 text-rose-600 dark:text-rose-400">Payment Verification</h4>
            <p className="text-slate-600 dark:text-slate-400 text-sm mb-4">
              Verify payments before proceeding with workflows
            </p>
            <pre className="bg-slate-100 dark:bg-slate-900 rounded-lg p-3 text-xs">
              <code className="text-rose-600 dark:text-rose-400">{`# Verify payment completed
mpp-near verify --tx-hash 0x123abc...`}</code>
            </pre>
          </div>

          <div className="bg-white dark:bg-slate-800/30 border border-slate-200 dark:border-slate-700 rounded-lg p-6 hover:border-teal-500/30 dark:hover:border-teal-500/20 transition-colors shadow-sm dark:shadow-none">
            <h4 className="text-lg font-bold mb-3 text-teal-600 dark:text-teal-400">Wallet Registration</h4>
            <p className="text-slate-600 dark:text-slate-400 text-sm mb-4">
              Set up agent custody wallets for gasless operations
            </p>
            <pre className="bg-slate-100 dark:bg-slate-900 rounded-lg p-3 text-xs">
              <code className="text-teal-600 dark:text-teal-400">{`# Register agent wallet
mpp-near register`}</code>
            </pre>
          </div>
        </div>

        <div className="mt-12 bg-gradient-to-r from-violet-500/10 to-rose-500/10 dark:from-violet-500/5 dark:to-rose-500/5 border border-violet-200 dark:border-violet-500/20 rounded-xl p-8">
          <div className="flex flex-col md:flex-row items-center justify-between gap-6">
            <div>
              <h4 className="text-xl font-bold mb-2 text-slate-900 dark:text-slate-100">Ready to Get Started?</h4>
              <p className="text-slate-600 dark:text-slate-400">
                Build from source to enable gasless payments for your AI agents.
              </p>
              <code className="mt-2 block text-sm text-slate-700 dark:text-slate-300 bg-slate-100 dark:bg-slate-800 px-3 py-1 rounded">
                git clone https://github.com/kampouse/mpp-near && cd mpp-near && cargo install --path . --features intents
              </code>
            </div>
            <div className="flex gap-4">
              <a
                href="https://github.com/kampouse/mpp-near"
                target="_blank"
                rel="noopener noreferrer"
                className="px-6 py-3 bg-slate-900 dark:bg-slate-50 hover:bg-slate-800 dark:hover:bg-slate-100 rounded-lg font-medium transition-all duration-200 text-white dark:text-slate-900"
              >
                View on GitHub
              </a>
            </div>
          </div>
        </div>
      </div>
    </section>
  )
}
