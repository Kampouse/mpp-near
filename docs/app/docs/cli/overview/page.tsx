export default function CLIOverviewPage() {
  return (
    <div>
      <h1 className="text-4xl font-bold mb-4 text-slate-900 dark:text-slate-50">CLI Reference</h1>
      <p className="text-xl text-slate-700 dark:text-slate-300 mb-8">
        Complete command-line interface reference for MPP-NEAR. Make gasless payments, swap tokens, and automate agent-to-agent transactions.
      </p>

      <div className="bg-violet-50 dark:bg-violet-500/10 border border-violet-200 dark:border-violet-500/20 rounded-lg p-6 mb-8">
        <h3 className="font-semibold text-violet-900 dark:text-violet-100 mb-2">For AI Agents & Automated Systems</h3>
        <ul className="text-sm text-violet-800 dark:text-violet-200 space-y-1">
          <li>✅ Gasless payments via OutLayer custody wallets</li>
          <li>✅ Cross-chain token swaps (20+ blockchains)</li>
          <li>✅ Agent-to-agent payment checks</li>
        </ul>
      </div>

      <h2 id="installation" className="text-2xl font-bold mb-4 text-slate-900 dark:text-slate-100 scroll-mt-20">Installation</h2>
      <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4 mb-8">
        <pre className="text-sm text-emerald-400">
          <code>{`git clone https://github.com/kampouse/mpp-near.git
cd mpp-near
cargo install --path . --features intents`}</code>
        </pre>
      </div>

      <h2 id="global-options" className="text-2xl font-bold mb-4 text-slate-900 dark:text-slate-100 scroll-mt-20">Global Options</h2>
      <div className="bg-slate-50 dark:bg-slate-900 rounded-lg p-4 mb-8 overflow-x-auto">
        <table className="w-full text-sm">
          <thead>
            <tr className="border-b border-slate-200 dark:border-slate-700">
              <th className="text-left py-2 text-slate-900 dark:text-slate-100">Option</th>
              <th className="text-left py-2 text-slate-900 dark:text-slate-100">Description</th>
            </tr>
          </thead>
          <tbody className="text-slate-700 dark:text-slate-300">
            <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">-m, --method</code></td><td>Payment method (standard|intents, default: intents)</td></tr>
            <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">-c, --config</code></td><td>Path to config file</td></tr>
            <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">--rpc-url</code></td><td>RPC URL for standard provider</td></tr>
            <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">-a, --account</code></td><td>Account ID for standard provider</td></tr>
            <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">-k, --private-key</code></td><td>Private key for standard provider</td></tr>
            <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">--api-key</code></td><td>API key for intents provider</td></tr>
            <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">-v, --verbose</code></td><td>Verbose output</td></tr>
          </tbody>
        </table>
      </div>

      <h2 id="commands" className="text-2xl font-bold mb-4 text-slate-900 dark:text-slate-100 scroll-mt-20">Commands</h2>

      <div className="mb-8">
        <h3 id="pay" className="text-xl font-semibold mb-2 text-slate-900 dark:text-slate-100 scroll-mt-20">mpp-near pay</h3>
        <p className="mb-4 text-slate-700 dark:text-slate-300">Send gasless payments to any NEAR account.</p>
        <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4 mb-4">
          <pre className="text-sm text-emerald-400">
            <code>{`# Basic payment (NEAR)
mpp-near pay --recipient merchant.near --amount 0.1

# Pay with USDC (gasless!)
mpp-near pay --recipient merchant.near --amount 10 --token usdc

# Pay with memo
mpp-near pay --recipient merchant.near --amount 1 --memo "Invoice #123"`}</code>
          </pre>
        </div>
        <div className="bg-slate-50 dark:bg-slate-900 rounded-lg p-4">
          <table className="w-full text-sm">
            <thead>
              <tr className="border-b border-slate-200 dark:border-slate-700">
                <th className="text-left py-2 text-slate-900 dark:text-slate-100">Option</th>
                <th className="text-left py-2 text-slate-900 dark:text-slate-100">Description</th>
              </tr>
            </thead>
            <tbody className="text-slate-700 dark:text-slate-300">
              <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">-r, --recipient</code></td><td>Recipient account ID</td></tr>
              <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">-n, --amount</code></td><td>Amount to send</td></tr>
              <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">-t, --token</code></td><td>Token (near|usdc|usdt, default: near)</td></tr>
              <tr><td className="py-1"><code className="bg-slate-200 dark:bg-slate-700 px-1 rounded">-o, --memo</code></td><td>Memo to include</td></tr>
            </tbody>
          </table>
        </div>
      </div>

      <div className="mb-8">
        <h3 id="register" className="text-xl font-semibold mb-2 text-slate-900 dark:text-slate-100 scroll-mt-20">mpp-near register</h3>
        <p className="mb-4 text-slate-700 dark:text-slate-300">Register a new OutLayer custody wallet for gasless operations.</p>
        <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4">
          <pre className="text-sm text-emerald-400">
            <code>{`mpp-near register`}</code>
          </pre>
        </div>
        <p className="mt-2 text-sm text-slate-600 dark:text-slate-400">
          Returns an API key required for gasless operations. Save it securely - it's shown only once.
        </p>
      </div>

      <div className="mb-8">
        <h3 id="balance" className="text-xl font-semibold mb-2 text-slate-900 dark:text-slate-100 scroll-mt-20">mpp-near balance</h3>
        <p className="mb-4 text-slate-700 dark:text-slate-300">Check wallet balances.</p>
        <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4">
          <pre className="text-sm text-emerald-400">
            <code>{`# Check own balance
mpp-near balance --api-key wk_...

# Check specific account
mpp-near balance --account user.near`}</code>
          </pre>
        </div>
      </div>

      <div className="mb-8">
        <h3 id="fund-link" className="text-xl font-semibold mb-2 text-slate-900 dark:text-slate-100 scroll-mt-20">mpp-near fund-link</h3>
        <p className="mb-4 text-slate-700 dark:text-slate-300">Generate a funding link for your wallet.</p>
        <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4">
          <pre className="text-sm text-emerald-400">
            <code>{`# Generate NEAR funding link
mpp-near fund-link --amount 0.1 --token near

# Generate USDC funding link
mpp-near fund-link --amount 10 --token usdc --memo "Top up"

# Fund to intents balance (for gasless operations)
mpp-near fund-link --amount 1 --token near --intents`}</code>
          </pre>
        </div>
        <p className="mt-2 text-sm text-slate-600 dark:text-slate-400">
          Opens a browser link to fund your wallet. Auto-registers storage for the recipient.
        </p>
      </div>

      <div className="mb-8">
        <h3 id="handoff" className="text-xl font-semibold mb-2 text-slate-900 dark:text-slate-100 scroll-mt-20">mpp-near handoff</h3>
        <p className="mb-4 text-slate-700 dark:text-slate-300">Show wallet management URL.</p>
        <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4">
          <pre className="text-sm text-emerald-400">
            <code>{`mpp-near handoff --api-key wk_...`}</code>
          </pre>
        </div>
        <p className="mt-2 text-sm text-slate-600 dark:text-slate-400">
          Use to configure spending policies, view transaction history, and set up multi-sig approval.
        </p>
      </div>

      <div className="mb-8">
        <h3 id="storage-deposit" className="text-xl font-semibold mb-2 text-slate-900 dark:text-slate-100 scroll-mt-20">mpp-near storage-deposit</h3>
        <p className="mb-4 text-slate-700 dark:text-slate-300">Register storage for an account (required before receiving tokens).</p>
        <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4">
          <pre className="text-sm text-emerald-400">
            <code>{`# Register storage for own account
mpp-near storage-deposit --token usdc

# Register storage for another account
mpp-near storage-deposit --account merchant.near --token usdc`}</code>
          </pre>
        </div>
      </div>

      <div className="mb-8">
        <h3 id="swap" className="text-xl font-semibold mb-2 text-slate-900 dark:text-slate-100 scroll-mt-20">mpp-near swap</h3>
        <p className="mb-4 text-slate-700 dark:text-slate-300">Swap tokens across 20+ blockchains (gasless).</p>
        <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4">
          <pre className="text-sm text-emerald-400">
            <code>{`# Swap NEAR to USDC
mpp-near swap --from near --to usdc --amount 1

# Swap USDT to NEAR
mpp-near swap --from usdt --to near --amount 10`}</code>
          </pre>
        </div>
      </div>

      <div className="mb-8">
        <h3 id="tokens" className="text-xl font-semibold mb-2 text-slate-900 dark:text-slate-100 scroll-mt-20">mpp-near tokens</h3>
        <p className="mb-4 text-slate-700 dark:text-slate-300">List available tokens for swapping.</p>
        <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4">
          <pre className="text-sm text-emerald-400">
            <code>{`mpp-near tokens --api-key wk_...`}</code>
          </pre>
        </div>
      </div>

      <div className="mb-8">
        <h3 id="create-check" className="text-xl font-semibold mb-2 text-slate-900 dark:text-slate-100 scroll-mt-20">mpp-near create-check</h3>
        <p className="mb-4 text-slate-700 dark:text-slate-300">Create a payment check (intents only).</p>
        <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4">
          <pre className="text-sm text-emerald-400">
            <code>{`# Create check for 10 USDC
mpp-near create-check --amount 10 --token usdc

# Create check with memo and expiry
mpp-near create-check --amount 10 --token usdc --memo "Payment" --expires-in 86400`}</code>
          </pre>
        </div>
        <p className="mt-2 text-sm text-slate-600 dark:text-slate-400">
          Returns a check key to share with the recipient. The check can be claimed using <code className="bg-slate-200 dark:bg-slate-700 px-1 rounded text-xs">claim-check</code>.
        </p>
      </div>

      <div className="mb-8">
        <h3 id="claim-check" className="text-xl font-semibold mb-2 text-slate-900 dark:text-slate-100 scroll-mt-20">mpp-near claim-check</h3>
        <p className="mb-4 text-slate-700 dark:text-slate-300">Claim a payment check (intents only).</p>
        <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4">
          <pre className="text-sm text-emerald-400">
            <code>{`# Claim entire check
mpp-near claim-check --check-key chk_...

# Claim partial amount
mpp-near claim-check --check-key chk_... --amount 5`}</code>
          </pre>
        </div>
      </div>

      <div className="mb-8">
        <h3 id="verify" className="text-xl font-semibold mb-2 text-slate-900 dark:text-slate-100 scroll-mt-20">mpp-near verify</h3>
        <p className="mb-4 text-slate-700 dark:text-slate-300">Verify a transaction.</p>
        <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4">
          <pre className="text-sm text-emerald-400">
            <code>{`# Verify transaction
mpp-near verify --tx-hash 0x...

# Verify with expected amount and recipient
mpp-near verify --tx-hash 0x... --expected-amount 0.1 --expected-recipient merchant.near`}</code>
          </pre>
        </div>
      </div>

      <div className="mb-8">
        <h3 id="server" className="text-xl font-semibold mb-2 text-slate-900 dark:text-slate-100 scroll-mt-20">mpp-near server</h3>
        <p className="mb-4 text-slate-700 dark:text-slate-300">Start a payment server.</p>
        <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4">
          <pre className="text-sm text-emerald-400">
            <code>{`# Start server on port 3000
mpp-near server --port 3000 --recipient wallet.near --min-amount 0.001`}</code>
          </pre>
        </div>
      </div>

      <div className="mb-8">
        <h3 id="config" className="text-xl font-semibold mb-2 text-slate-900 dark:text-slate-100 scroll-mt-20">mpp-near config</h3>
        <p className="mb-4 text-slate-700 dark:text-slate-300">Show current configuration.</p>
        <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4">
          <pre className="text-sm text-emerald-400">
            <code>{`mpp-near config`}</code>
          </pre>
        </div>
      </div>

      <h2 id="config-file" className="text-2xl font-bold mb-4 text-slate-900 dark:text-slate-100 scroll-mt-20">Configuration File</h2>
      <p className="mb-4 text-slate-700 dark:text-slate-300">
        Create a config file at <code className="bg-slate-100 dark:bg-slate-800 px-2 py-0.5 rounded font-mono text-sm">~/.mpp-near/config.toml</code> or <code className="bg-slate-100 dark:bg-slate-800 px-2 py-0.5 rounded font-mono text-sm">.mpp-config</code>:
      </p>
      <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4 mb-8">
        <pre className="text-sm text-emerald-400">
          <code>{`method = "intents"

[intents]
api_key = "wk_..."
api_url = "https://api.outlayer.fastnear.com"

[standard]
account = "user.near"
private_key = "ed25519:..."
rpc_url = "https://rpc.mainnet.near.org"`}</code>
        </pre>
      </div>

      <h2 id="environment-variables" className="text-2xl font-bold mb-4 text-slate-900 dark:text-slate-100 scroll-mt-20">Environment Variables</h2>
      <ul className="list-disc pl-6 space-y-2 text-slate-700 dark:text-slate-300 mb-8">
        <li><code className="bg-slate-100 dark:bg-slate-800 px-2 py-0.5 rounded text-emerald-700 dark:text-emerald-400 font-mono">MPP_NEAR_API_KEY</code> - OutLayer API key for gasless operations</li>
        <li><code className="bg-slate-100 dark:bg-slate-800 px-2 py-0.5 rounded text-emerald-700 dark:text-emerald-400 font-mono">MPP_NEAR_QUIET</code> - Suppress info messages</li>
      </ul>

      <div className="mt-8 p-6 bg-slate-50 dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg">
        <h4 className="font-semibold text-slate-900 dark:text-slate-100 mb-2">See Also</h4>
        <ul className="space-y-2 text-sm">
          <li><a href="/docs/outlayer" className="hover:underline" style={{color: '#00C08B'}}>OutLayer Integration →</a></li>
          <li><a href="/docs/quickstart" className="hover:underline" style={{color: '#00C08B'}}>Quick Start →</a></li>
        </ul>
      </div>
    </div>
  )
}
