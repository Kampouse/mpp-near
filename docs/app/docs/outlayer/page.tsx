export default function OutlayerPage() {
  return (
    <div>
      <h1 className="text-4xl font-bold mb-4 text-slate-900 dark:text-slate-50">OutLayer Integration</h1>
      <p className="text-xl text-slate-700 dark:text-slate-300 mb-8">
        OutLayer provides verifiable off-chain computation for NEAR Blockchain using Trusted Execution Environments (TEE) with Intel TDX attestation. Enable agents to transact without managing gas or private keys while ensuring cryptographic proof of execution.
      </p>

      <div className="bg-amber-50 dark:bg-amber-500/10 border border-amber-200 dark:border-amber-500/20 rounded-lg p-6 mb-8">
        <h3 className="font-semibold text-amber-900 dark:text-amber-100 mb-2">What is OutLayer?</h3>
        <p className="text-amber-800 dark:text-amber-200 text-sm mb-4">
          OutLayer is a verifiable computation layer that runs code in Trusted Execution Environments (TEEs). It provides cryptographic proofs of execution using Intel TDX attestation, meaning no trust is required — you can verify everything cryptographically.
        </p>
        <a
          href="https://outlayer.fastnear.com"
          target="_blank"
          rel="noopener noreferrer"
          className="text-amber-700 dark:text-amber-400 hover:text-amber-800 dark:hover:text-amber-300 text-sm font-medium"
        >
          Visit OutLayer →
        </a>
      </div>

      <h2 className="text-2xl font-bold mb-4 text-slate-900 dark:text-slate-100">Key Features</h2>

      <div className="grid md:grid-cols-2 gap-6 mb-8">
        <div className="border border-slate-200 dark:border-slate-800 rounded-lg p-6">
          <h3 className="font-semibold text-lg text-slate-900 dark:text-slate-100 mb-2">🔐 Verifiable Execution</h3>
          <p className="text-slate-700 dark:text-slate-300 text-sm">
            Every computation is executed in a Trusted Execution Environment (TEE) with Intel TDX attestation. Get cryptographic proofs that your code ran exactly as specified — no trust required.
          </p>
        </div>

        <div className="border border-slate-200 dark:border-slate-800 rounded-lg p-6">
          <h3 className="font-semibold text-lg text-slate-900 dark:text-slate-100 mb-2">💰 Monetize Your API</h3>
          <p className="text-slate-700 dark:text-slate-300 text-sm">
            Earn gasless stablecoin payments for your API services. OutLayer handles custody wallets and gas fees, so you can focus on building great APIs.
          </p>
        </div>

        <div className="border border-slate-200 dark:border-slate-800 rounded-lg p-6">
          <h3 className="font-semibold text-lg text-slate-900 dark:text-slate-100 mb-2">🔑 Upgradeable TEE Vault</h3>
          <p className="text-slate-700 dark:text-slate-300 text-sm">
            Private keys live securely inside the TEE and persist across upgrades. Update your code without losing access to funds or credentials.
          </p>
        </div>

        <div className="border border-slate-200 dark:border-slate-800 rounded-lg p-6">
          <h3 className="font-semibold text-lg text-slate-900 dark:text-slate-100 mb-2">⚡ Gasless Operations</h3>
          <p className="text-slate-700 dark:text-slate-300 text-sm">
            All gas fees are handled via NEAR Intents. Agents and automated systems can transact without managing gas or private keys.
          </p>
        </div>
      </div>

      <h2 className="text-2xl font-bold mb-4 text-slate-900 dark:text-slate-100">Use Cases</h2>

      <div className="space-y-4 mb-8">
        <div className="bg-slate-50 dark:bg-slate-800 border-l-4 border-emerald-500 p-4 rounded-r-lg">
          <h4 className="font-semibold text-slate-900 dark:text-slate-100 mb-1">AI Inference</h4>
          <p className="text-slate-700 dark:text-slate-300 text-sm">
            Run AI models in a verifiable environment. Users get cryptographic proofs that inference was performed correctly.
          </p>
        </div>

        <div className="bg-slate-50 dark:bg-slate-800 border-l-4 border-violet-500 p-4 rounded-r-lg">
          <h4 className="font-semibold text-slate-900 dark:text-slate-100 mb-1">Secure Randomness</h4>
          <p className="text-slate-700 dark:text-slate-300 text-sm">
            Generate verifiable random numbers inside TEE. Perfect for gaming, gambling, and any application requiring fair randomness.
          </p>
        </div>

        <div className="bg-slate-50 dark:bg-slate-800 border-l-4 border-amber-500 p-4 rounded-r-lg">
          <h4 className="font-semibold text-slate-900 dark:text-slate-100 mb-1">HTTP APIs & Webhooks</h4>
          <p className="text-slate-700 dark:text-slate-300 text-sm">
            Build payment-gated APIs with HTTP 402. Monetize per-request while ensuring all computation is verifiable.
          </p>
        </div>

        <div className="bg-slate-50 dark:bg-slate-800 border-l-4 border-blue-500 p-4 rounded-r-lg">
          <h4 className="font-semibold text-slate-900 dark:text-slate-100 mb-1">Gasless Custody Wallets</h4>
          <p className="text-slate-700 dark:text-slate-300 text-sm">
            Enable AI agents to transact without managing gas or private keys. Multi-token support (NEAR, USDC, USDT, and more).
          </p>
        </div>
      </div>

      <h2 className="text-2xl font-bold mb-4 text-slate-900 dark:text-slate-100">Getting Started with MPP-NEAR</h2>

      <h3 className="text-xl font-semibold mb-2 text-slate-900 dark:text-slate-100">1. Register for an API Key</h3>
      <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4 mb-6">
        <pre className="text-sm text-emerald-400">
          <code>{`mpp-near register`}</code>
        </pre>
      </div>

      <h3 className="text-xl font-semibold mb-2 text-slate-900 dark:text-slate-100">2. Fund Your Wallet</h3>
      <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4 mb-6">
        <pre className="text-sm text-emerald-400">
          <code>{`mpp-near fund-link --amount 0.1 --token near`}</code>
        </pre>
      </div>

      <h3 className="text-xl font-semibold mb-2 text-slate-900 dark:text-slate-100">3. Make Gasless Payments</h3>
      <div className="bg-slate-900 dark:bg-slate-950 rounded-lg p-4 mb-8">
        <pre className="text-sm text-emerald-400">
          <code>{`mpp-near pay --recipient merchant.near --amount 10 --token usdc`}</code>
        </pre>
      </div>

      <h2 className="text-2xl font-bold mb-4 text-slate-900 dark:text-slate-100">Why OutLayer?</h2>
      <ul className="list-disc pl-6 space-y-2 mb-8 text-slate-700 dark:text-slate-300">
        <li><strong className="text-slate-900 dark:text-slate-100">Zero Trust Required:</strong> All computations are verifiable through Intel TDX attestation</li>
        <li><strong className="text-slate-900 dark:text-slate-100">No Gas Management:</strong> All gas fees handled via NEAR Intents</li>
        <li><strong className="text-slate-900 dark:text-slate-100">Multi-Token Support:</strong> NEAR, USDC, USDT, and more</li>
        <li><strong className="text-slate-900 dark:text-slate-100">Upgradeable:</strong> Private keys persist across code upgrades</li>
        <li><strong className="text-slate-900 dark:text-slate-100">Perfect for AI Agents:</strong> HTTP-based API access, no private key management</li>
      </ul>

      <div className="mt-8 p-6 bg-slate-50 dark:bg-slate-800 border border-slate-200 dark:border-slate-700 rounded-lg">
        <h4 className="font-semibold text-slate-900 dark:text-slate-100 mb-2">See Also</h4>
        <ul className="space-y-2 text-sm">
          <li><a href="/docs/cli/overview" className="text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300">CLI Reference →</a></li>
          <li><a href="/docs/quickstart" className="text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300">Quick Start →</a></li>
        </ul>
      </div>
    </div>
  )
}
