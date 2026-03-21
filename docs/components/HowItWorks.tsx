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
      description: "Create a gasless custody wallet via OutLayer"
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
          <h2 className="text-3xl md:text-4xl font-bold mb-4 text-slate-900 dark:text-slate-50">
            Quick Start
          </h2>
          <p className="text-slate-600 dark:text-slate-400">
            Get started with gasless NEAR payments in minutes
          </p>
        </div>

        <div className="space-y-6">
          {steps.map((step, i) => (
            <div
              key={i}
              className="flex flex-col md:flex-row items-start md:items-center gap-4 p-6 bg-white dark:bg-slate-800/30 rounded-xl border border-slate-200 dark:border-slate-700 hover:border-emerald-500/30 dark:hover:border-emerald-500/20 transition-all duration-200 shadow-sm dark:shadow-none"
            >
              <div className="flex-shrink-0 w-12 h-12 bg-emerald-50 dark:bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 rounded-lg flex items-center justify-center font-bold text-xl">
                {step.step}
              </div>
              <div className="flex-grow">
                <h3 className="font-semibold mb-1 text-slate-900 dark:text-slate-100">{step.title}</h3>
                <p className="text-slate-600 dark:text-slate-400 text-sm">{step.description}</p>
              </div>
              <code className="flex-shrink-0 px-4 py-2 bg-slate-100 dark:bg-slate-800 rounded-lg text-sm font-mono text-emerald-600 dark:text-emerald-400">
                {step.code}
              </code>
            </div>
          ))}
        </div>
      </div>
    </section>
  )
}
