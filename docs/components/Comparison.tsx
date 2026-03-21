export default function Comparison() {
  const comparisons = [
    {
      feature: "Gasless Transactions",
      standard: "❌ Requires NEAR for gas",
      intents: "✅ Zero gas fees"
    },
    {
      feature: "Token Swaps",
      standard: "❌ Not supported",
      intents: "✅ 20+ chains"
    },
    {
      feature: "Cross-Chain",
      standard: "❌ Limited to NEAR",
      intents: "✅ Native bridges"
    },
    {
      feature: "Agent Payments",
      standard: "❌ Storage required",
      intents: "✅ Payment checks"
    },
    {
      feature: "Setup Complexity",
      standard: "High (key management)",
      intents: "Low (API key only)"
    },
    {
      feature: "Transaction Speed",
      standard: "~1s (block time)",
      intents: "Instant (solver relay)"
    }
  ]

  return (
    <section id="comparison" className="py-20 px-6">
      <div className="max-w-6xl mx-auto">
        <div className="text-center mb-12">
          <h2 className="text-3xl md:text-4xl font-bold mb-4 text-slate-900 dark:text-slate-50">
            Payment Providers
          </h2>
          <p className="text-slate-600 dark:text-slate-400">
            MPP-NEAR supports both standard NEAR transactions and gasless Intents
          </p>
        </div>

        <div className="overflow-x-auto">
          <table className="w-full">
            <thead>
              <tr className="border-b border-slate-200 dark:border-slate-800">
                <th className="text-left py-4 px-4 font-medium text-slate-600 dark:text-slate-400">Feature</th>
                <th className="text-left py-4 px-4 font-medium text-slate-600 dark:text-slate-400">Standard Provider</th>
                <th className="text-left py-4 px-4 font-medium text-violet-600 dark:text-violet-400">Intents Provider</th>
              </tr>
            </thead>
            <tbody>
              {comparisons.map((row, i) => (
                <tr key={i} className="border-b border-slate-100 dark:border-slate-800/50 hover:bg-slate-50 dark:hover:bg-slate-800/20 transition-colors">
                  <td className="py-4 px-4 font-medium text-slate-900 dark:text-slate-100">{row.feature}</td>
                  <td className="py-4 px-4 text-slate-600 dark:text-slate-400">{row.standard}</td>
                  <td className="py-4 px-4 text-slate-900 dark:text-slate-100">{row.intents}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>

        <div className="mt-8 p-6 bg-white dark:bg-slate-800/30 rounded-xl border border-slate-200 dark:border-slate-700 shadow-sm dark:shadow-none">
          <h3 className="font-semibold mb-2 text-slate-900 dark:text-slate-100">When to Use Each</h3>
          <div className="grid md:grid-cols-2 gap-6 text-sm text-slate-600 dark:text-slate-400">
            <div>
              <div className="font-medium text-slate-900 dark:text-slate-100 mb-2">Standard Provider</div>
              <ul className="space-y-1">
                <li>• You need full control over keys</li>
                <li>• On-chain transactions required</li>
                <li>• No external dependencies</li>
              </ul>
            </div>
            <div>
              <div className="font-medium text-slate-900 dark:text-slate-100 mb-2">Intents Provider</div>
              <ul className="space-y-1">
                <li>• Gasless microtransactions</li>
                <li>• Cross-chain operations</li>
                <li>• Agent-to-agent payments</li>
              </ul>
            </div>
          </div>
        </div>
      </div>
    </section>
  )
}
