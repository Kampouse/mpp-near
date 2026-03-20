export default function WhyUseIt() {
  const useCases = [
    {
      title: "API Monetization",
      description: "Charge per API call with automatic payment handling. Clients pay before accessing your endpoints.",
      icon: "⚡"
    },
    {
      title: "Autonomous Agents",
      description: "AI agents can pay for services automatically using payment checks or gasless transactions.",
      icon: "🤖"
    },
    {
      title: "Microtransactions",
      description: "Send payments as small as $0.01 without gas fees blocking the transaction.",
      icon: "💰"
    },
    {
      title: "Cross-Chain Operations",
      description: "Swap and pay with tokens from 20+ blockchains through NEAR Intents.",
      icon: "🔄"
    },
    {
      title: "No Gas Management",
      description: "Intents provider handles all gas costs. No need to maintain NEAR for transaction fees.",
      icon: "⛽"
    },
    {
      title: "Machine Commerce",
      description: "Enable IoT devices, scripts, and automated systems to pay for resources.",
      icon: "🔧"
    }
  ]

  return (
    <section id="why-use" className="py-20 px-6">
      <div className="max-w-6xl mx-auto">
        <div className="text-center mb-12">
          <h2 className="text-3xl md:text-4xl font-bold mb-4">
            Why Use MPP-NEAR?
          </h2>
          <p className="text-gray-400 max-w-2xl mx-auto">
            Enable machine-to-machine commerce without traditional payment friction
          </p>
        </div>
        
        <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
          {useCases.map((useCase, i) => (
            <div 
              key={i}
              className="p-6 bg-near-gray/30 rounded-xl border border-gray-800 hover:border-gray-700 transition-colors"
            >
              <div className="text-3xl mb-4">{useCase.icon}</div>
              <h3 className="text-lg font-semibold mb-2">{useCase.title}</h3>
              <p className="text-gray-400 text-sm leading-relaxed">
                {useCase.description}
              </p>
            </div>
          ))}
        </div>
      </div>
    </section>
  )
}
