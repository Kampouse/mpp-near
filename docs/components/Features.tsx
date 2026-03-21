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
        "Agent-to-agent payment checks",
        "Balance caching",
        "Configurable expiration"
      ]
    }
  ]

  return (
    <section id="features" className="py-20 px-6 bg-slate-50 dark:bg-slate-900/20">
      <div className="max-w-6xl mx-auto">
        <div className="text-center mb-12">
          <h2 className="text-3xl md:text-4xl font-bold mb-4 text-slate-900 dark:text-slate-50">
            Features
          </h2>
          <p className="text-slate-600 dark:text-slate-400">
            Comprehensive toolkit for NEAR payments
          </p>
        </div>

        <div className="grid md:grid-cols-2 lg:grid-cols-4 gap-6">
          {features.map((feature, i) => (
            <div key={i} className="bg-white dark:bg-slate-800/30 rounded-xl p-6 border border-slate-200 dark:border-slate-700 hover:border-[#00C08B]/30 dark:hover:border-[#00C08B]/20 transition-all duration-200 shadow-sm dark:shadow-none">
              <h3 className="font-semibold mb-4" style={{color: '#00C08B'}}>{feature.category}</h3>
              <ul className="space-y-3">
                {feature.items.map((item, j) => (
                  <li key={j} className="flex items-start gap-2 text-sm text-slate-600 dark:text-slate-400">
                    <span className="mt-0.5 flex-shrink-0" style={{color: '#00C08B'}}>
                      <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                      </svg>
                    </span>
                    {item}
                  </li>
                ))}
              </ul>
            </div>
          ))}
        </div>

        <div className="mt-8 text-center text-sm text-slate-500 dark:text-slate-400">
          Gasless operations powered by{" "}
          <a href="https://outlayer.fastnear.com" target="_blank" rel="noopener noreferrer" className="hover:underline font-medium" style={{color: '#00C08B'}}>
            OutLayer
          </a>{" "}
          custody wallets
        </div>
      </div>
    </section>
  )
}
