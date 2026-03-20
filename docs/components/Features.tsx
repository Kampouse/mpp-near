export default function Features() {
  const features = [
    {
      category: "Payments",
      items: [
        "NEAR token payments",
        "NEP-141 token support (USDC, USDT, etc.)",
        "Gasless transfers via NEAR Intents",
        "Agent-to-agent payment checks"
      ]
    },
    {
      category: "Protocol",
      items: [
        "HTTP 402 standard implementation",
        "Challenge-response authentication",
        "Replay protection",
        "Payment verification"
      ]
    },
    {
      category: "Developer Tools",
      items: [
        "Rust SDK",
        "CLI tool for all operations",
        "Axum extractors",
        "Middleware for popular frameworks"
      ]
    },
    {
      category: "Advanced",
      items: [
        "Cross-chain swaps (20+ chains)",
        "Payment channels",
        "Balance caching",
        "Configurable expiration"
      ]
    }
  ]

  return (
    <section id="features" className="py-20 px-6 bg-near-gray/50">
      <div className="max-w-6xl mx-auto">
        <div className="text-center mb-12">
          <h2 className="text-3xl md:text-4xl font-bold mb-4">
            Features
          </h2>
          <p className="text-gray-400">
            Comprehensive toolkit for NEAR payments
          </p>
        </div>
        
        <div className="grid md:grid-cols-2 lg:grid-cols-4 gap-6">
          {features.map((feature, i) => (
            <div key={i} className="bg-near-dark rounded-xl p-6 border border-gray-800">
              <h3 className="font-semibold mb-4 text-blue-400">{feature.category}</h3>
              <ul className="space-y-3">
                {feature.items.map((item, j) => (
                  <li key={j} className="flex items-start gap-2 text-sm text-gray-400">
                    <span className="text-green-400 mt-1">✓</span>
                    {item}
                  </li>
                ))}
              </ul>
            </div>
          ))}
        </div>
      </div>
    </section>
  )
}
