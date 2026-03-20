export default function HowItWorks() {
  const steps = [
    {
      step: "1",
      title: "Install",
      code: "cargo install mpp-near --features intents",
      description: "Install the CLI tool with gasless payment support"
    },
    {
      step: "2", 
      title: "Register",
      code: "mpp-near register",
      description: "Create a gasless custody wallet via NEAR Intents"
    },
    {
      step: "3",
      title: "Fund",
      code: "mpp-near fund-link --amount 0.1 --token near",
      description: "Generate a link to deposit NEAR into your wallet"
    },
    {
      step: "4",
      title: "Pay",
      code: "mpp-near pay --recipient merchant.near --amount 1",
      description: "Send gasless payments to any NEAR account"
    }
  ]

  return (
    <section id="how-it-works" className="py-20 px-6">
      <div className="max-w-6xl mx-auto">
        <div className="text-center mb-12">
          <h2 className="text-3xl md:text-4xl font-bold mb-4">
            Quick Start
          </h2>
          <p className="text-gray-400">
            Get started with gasless NEAR payments in minutes
          </p>
        </div>
        
        <div className="space-y-6">
          {steps.map((step, i) => (
            <div 
              key={i}
              className="flex flex-col md:flex-row items-start md:items-center gap-4 p-6 bg-near-gray/30 rounded-xl border border-gray-800"
            >
              <div className="flex-shrink-0 w-12 h-12 bg-blue-500/20 text-blue-400 rounded-lg flex items-center justify-center font-bold text-xl">
                {step.step}
              </div>
              <div className="flex-grow">
                <h3 className="font-semibold mb-1">{step.title}</h3>
                <p className="text-gray-400 text-sm">{step.description}</p>
              </div>
              <code className="flex-shrink-0 px-4 py-2 bg-near-dark rounded-lg text-sm font-mono text-green-400">
                {step.code}
              </code>
            </div>
          ))}
        </div>
      </div>
    </section>
  )
}
