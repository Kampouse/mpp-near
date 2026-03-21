export default function WhyUseIt() {
  return (
    <section id="why-use" className="py-24 px-6">
      <div className="max-w-6xl mx-auto">
        <div className="text-center mb-16">
          <h2 className="text-3xl md:text-4xl font-bold mb-4 text-slate-900 dark:text-slate-50">
            Why MPP-NEAR?
          </h2>
          <p className="text-slate-600 dark:text-slate-400 max-w-2xl mx-auto">
            Enable machine-to-machine commerce without traditional payment friction
          </p>
        </div>

        <div className="bg-slate-50 dark:bg-slate-800/20 border border-slate-200 dark:border-slate-700 rounded-xl p-8 mb-16">
          <div className="grid md:grid-cols-2 gap-8">
            <div className="flex items-start gap-4">
              <div className="w-10 h-10 rounded-lg flex items-center justify-center flex-shrink-0" style={{backgroundColor: 'rgba(0, 192, 139, 0.1)'}}>
                <svg className="w-5 h-5" style={{color: '#00C08B'}} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2m-2-4h.01M17 16h.01" />
                </svg>
              </div>
              <div>
                <h4 className="font-semibold text-slate-900 dark:text-slate-100 mb-2">For Service Providers</h4>
                <p className="text-slate-600 dark:text-slate-400 text-sm leading-relaxed">
                  Use the Rust API to build payment-gated APIs and monetize your services automatically with HTTP 402.
                </p>
              </div>
            </div>
            <div className="flex items-start gap-4">
              <div className="w-10 h-10 rounded-lg flex items-center justify-center flex-shrink-0" style={{backgroundColor: 'rgba(99, 102, 241, 0.1)'}}>
                <svg className="w-5 h-5" style={{color: '#6366F1'}} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
                </svg>
              </div>
              <div>
                <h4 className="font-semibold text-slate-900 dark:text-slate-100 mb-2">For Agents & Clients</h4>
                <p className="text-slate-600 dark:text-slate-400 text-sm leading-relaxed">
                  Use the CLI to enable AI agents, scripts, and automated systems to pay for services gaslessly.
                </p>
              </div>
            </div>
          </div>
        </div>

        <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
          <div className="bg-white dark:bg-slate-800/30 border border-slate-200 dark:border-slate-700 rounded-xl p-6 hover:border-[#00C08B]/30 dark:hover:border-[#00C08B]/20 transition-colors shadow-sm dark:shadow-none">
            <div className="w-10 h-10 rounded-lg flex items-center justify-center mb-4" style={{backgroundColor: 'rgba(0, 192, 139, 0.1)'}}>
              <svg className="w-5 h-5" style={{color: '#00C08B'}} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
            <h3 className="text-lg font-semibold mb-2 text-slate-900 dark:text-slate-100">Near-Zero Fees</h3>
            <p className="text-slate-600 dark:text-slate-400 text-sm leading-relaxed">
              Pay ~$0.001 per transaction instead of 2.9% + $0.30 with traditional processors
            </p>
          </div>

          <div className="bg-white dark:bg-slate-800/30 border border-slate-200 dark:border-slate-700 rounded-xl p-6 hover:border-[#F97316]/30 dark:hover:border-[#F97316]/20 transition-colors shadow-sm dark:shadow-none">
            <div className="w-10 h-10 rounded-lg flex items-center justify-center mb-4" style={{backgroundColor: 'rgba(249, 115, 22, 0.1)'}}>
              <svg className="w-5 h-5" style={{color: '#F97316'}} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
              </svg>
            </div>
            <h3 className="text-lg font-semibold mb-2 text-slate-900 dark:text-slate-100">Gasless Transactions</h3>
            <p className="text-slate-600 dark:text-slate-400 text-sm leading-relaxed">
              <a href="https://outlayer.fastnear.com" target="_blank" rel="noopener noreferrer" className="hover:underline" style={{color: '#F97316'}}>OutLayer</a> handles all gas via NEAR Intents. No need to maintain NEAR for fees
            </p>
          </div>

          <div className="bg-white dark:bg-slate-800/30 border border-slate-200 dark:border-slate-700 rounded-xl p-6 hover:border-[#6366F1]/30 dark:hover:border-[#6366F1]/20 transition-colors shadow-sm dark:shadow-none">
            <div className="w-10 h-10 rounded-lg flex items-center justify-center mb-4" style={{backgroundColor: 'rgba(99, 102, 241, 0.1)'}}>
              <svg className="w-5 h-5" style={{color: '#6366F1'}} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
            <h3 className="text-lg font-semibold mb-2 text-slate-900 dark:text-slate-100">20+ Blockchains</h3>
            <p className="text-slate-600 dark:text-slate-400 text-sm leading-relaxed">
              Swap and pay with tokens from NEAR, Ethereum, Bitcoin, Solana, and more via Intents
            </p>
          </div>

          <div className="bg-white dark:bg-slate-800/30 border border-slate-200 dark:border-slate-700 rounded-xl p-6 hover:border-[#22C55E]/30 dark:hover:border-[#22C55E]/20 transition-colors shadow-sm dark:shadow-none">
            <div className="w-10 h-10 rounded-lg flex items-center justify-center mb-4" style={{backgroundColor: 'rgba(34, 197, 94, 0.1)'}}>
              <svg className="w-5 h-5" style={{color: '#22C55E'}} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
              </svg>
            </div>
            <h3 className="text-lg font-semibold mb-2 text-slate-900 dark:text-slate-100">Stateless Security</h3>
            <p className="text-slate-600 dark:text-slate-400 text-sm leading-relaxed">
              HMAC binding lets you verify payments without database storage. Scale horizontally.
            </p>
          </div>

          <div className="bg-white dark:bg-slate-800/30 border border-slate-200 dark:border-slate-700 rounded-xl p-6 hover:border-[#EC4899]/30 dark:hover:border-[#EC4899]/20 transition-colors shadow-sm dark:shadow-none">
            <div className="w-10 h-10 rounded-lg flex items-center justify-center mb-4" style={{backgroundColor: 'rgba(236, 72, 153, 0.1)'}}>
              <svg className="w-5 h-5" style={{color: '#EC4899'}} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
              </svg>
            </div>
            <h3 className="text-lg font-semibold mb-2 text-slate-900 dark:text-slate-100">Instant Settlement</h3>
            <p className="text-slate-600 dark:text-slate-400 text-sm leading-relaxed">
              Payments settle in seconds, not days. No waiting for bank transfers or chargebacks
            </p>
          </div>

          <div className="bg-white dark:bg-slate-800/30 border border-slate-200 dark:border-slate-700 rounded-xl p-6 hover:border-[#06B6D4]/30 dark:hover:border-[#06B6D4]/20 transition-colors shadow-sm dark:shadow-none">
            <div className="w-10 h-10 rounded-lg flex items-center justify-center mb-4" style={{backgroundColor: 'rgba(6, 182, 212, 0.1)'}}>
              <svg className="w-5 h-5" style={{color: '#06B6D4'}} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
              </svg>
            </div>
            <h3 className="text-lg font-semibold mb-2 text-slate-900 dark:text-slate-100">Open Standard</h3>
            <p className="text-slate-600 dark:text-slate-400 text-sm leading-relaxed">
              Implements MPP-1.0 specification. Interoperable with any MPP implementation
            </p>
          </div>
        </div>
      </div>
    </section>
  )
}
