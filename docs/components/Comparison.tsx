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
          <h2 className="text-3xl md:text-4xl font-bold mb-4">
            Standard vs Intents
          </h2>
          <p className="text-gray-400">
            Choose the provider that fits your needs
          </p>
        </div>
        
        <div className="overflow-x-auto">
          <table className="w-full">
            <thead>
              <tr className="border-b border-gray-800">
                <th className="text-left py-4 px-4 font-medium text-gray-400">Feature</th>
                <th className="text-left py-4 px-4 font-medium text-gray-400">Standard Provider</th>
                <th className="text-left py-4 px-4 font-medium text-blue-400">Intents Provider</th>
              </tr>
            </thead>
            <tbody>
              {comparisons.map((row, i) => (
                <tr key={i} className="border-b border-gray-800/50">
                  <td className="py-4 px-4 font-medium">{row.feature}</td>
                  <td className="py-4 px-4 text-gray-400">{row.standard}</td>
                  <td className="py-4 px-4">{row.intents}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
        
        <div className="mt-8 p-6 bg-near-gray/30 rounded-xl border border-gray-800">
          <h3 className="font-semibold mb-2">When to Use Each</h3>
          <div className="grid md:grid-cols-2 gap-6 text-sm text-gray-400">
            <div>
              <div className="font-medium text-white mb-2">Standard Provider</div>
              <ul className="space-y-1">
                <li>• You need full control over keys</li>
                <li>• On-chain transactions required</li>
                <li>• No external dependencies</li>
              </ul>
            </div>
            <div>
              <div className="font-medium text-white mb-2">Intents Provider</div>
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
