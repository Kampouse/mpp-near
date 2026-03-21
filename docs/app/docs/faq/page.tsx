export default function FAQPage() {
  const faqs = [
    {
      q: "What is MPP-NEAR?",
      a: "MPP-NEAR implements the Machine Payments Protocol (MPP) for NEAR blockchain. It enables HTTP 402 payments for API monetization."
    },
    {
      q: "How is this different from Stripe?",
      a: "MPP offers near-zero fees (~$0.001 vs 2.9% + $0.30), instant settlement, and works globally without intermediaries."
    },
    {
      q: "Do I need a database?",
      a: "No! MPP uses HMAC binding for stateless verification. You can verify payments without storing state."
    },
    {
      q: "What is OutLayer?",
      a: "OutLayer provides custody wallets for gasless NEAR transactions via NEAR Intents. It handles gas so agents don't need NEAR."
    },
    {
      q: "Can AI agents use this?",
      a: "Yes! The CLI is designed for autonomous agents. No private key management - just use an API key."
    },
    {
      q: "What tokens are supported?",
      a: "NEAR, USDC, USDT, and 100+ tokens across 20+ blockchains."
    }
  ]

  return (
    <div>
      <h1 className="text-4xl font-bold mb-4 text-slate-900 dark:text-slate-50">Frequently Asked Questions</h1>
      <p className="text-xl text-slate-700 dark:text-slate-300 mb-12">
        Common questions about MPP-NEAR, HTTP 402 payments, and OutLayer integration.
      </p>

      <div className="space-y-6 mb-12">
        {faqs.map((faq, idx) => (
          <div key={idx} className="border border-slate-200 dark:border-slate-800 rounded-lg overflow-hidden">
            <div className="bg-slate-50 dark:bg-slate-800 p-4">
              <h3 className="font-semibold text-lg text-slate-900 dark:text-slate-100">{faq.q}</h3>
            </div>
            <div className="p-4 bg-white dark:bg-slate-900">
              <p className="text-slate-700 dark:text-slate-300">{faq.a}</p>
            </div>
          </div>
        ))}
      </div>

      <div className="p-6 bg-gradient-to-r from-violet-50 to-rose-50 dark:from-violet-950 dark:to-rose-950 border border-violet-200 dark:border-violet-900 rounded-lg">
        <h3 className="font-semibold text-slate-900 dark:text-slate-100 mb-2">Still Have Questions?</h3>
        <p className="text-sm text-slate-700 dark:text-slate-300 mb-4">
          Join our community or check out the GitHub repository for more help.
        </p>
        <div className="flex gap-4">
          <a
            href="https://github.com/kampouse/mpp-near/issues"
            target="_blank"
            rel="noopener noreferrer"
            className="text-violet-700 dark:text-violet-400 hover:text-violet-800 dark:hover:text-violet-300 text-sm font-medium"
          >
            GitHub Issues →
          </a>
          <a
            href="https://discord.gg/clawd"
            target="_blank"
            rel="noopener noreferrer"
            className="text-violet-700 dark:text-violet-400 hover:text-violet-800 dark:hover:text-violet-300 text-sm font-medium"
          >
            Discord →
          </a>
        </div>
      </div>
    </div>
  )
}
