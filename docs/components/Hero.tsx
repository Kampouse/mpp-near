import PixelArtLogo from './PixelArtLogo'

export default function Hero() {
  return (
    <section className="pt-32 pb-24 px-6">
      <div className="max-w-6xl mx-auto">
        <div className="text-center mb-16">
          <div className="inline-flex items-center gap-2 px-4 py-2 bg-slate-100 dark:bg-slate-800/50 rounded-full text-sm text-slate-600 dark:text-slate-400 mb-8">
            <span className="w-2 h-2 rounded-full animate-pulse" style={{backgroundColor: '#00C08B'}} />
            Open Standard • NEAR Blockchain • Gasless Payments
          </div>

          <div className="mb-6">
            <PixelArtLogo />
          </div>

          <h2 className="text-3xl md:text-5xl font-bold mb-8 tracking-tight text-slate-900 dark:text-slate-50">
            Machine Payments Protocol
          </h2>

          <p className="text-xl text-slate-600 dark:text-slate-400 max-w-2xl mx-auto mb-12 leading-relaxed">
            MPP-NEAR implements the open standard for machine-to-machine payments.
            Enable HTTP 402 payments for your APIs and empower autonomous agents to transact automatically.
            <span className="block mt-2 text-sm">Powered by <a href="https://outlayer.fastnear.com" target="_blank" rel="noopener noreferrer" style={{color: '#00C08B'}} className="hover:underline">OutLayer</a> for gasless custody wallets.</span>
          </p>

          <div className="flex flex-col sm:flex-row items-center justify-center gap-4">
            <a
              href="/docs/quickstart"
              className="px-8 py-3.5 bg-slate-900 dark:bg-slate-50 hover:bg-slate-800 dark:hover:bg-slate-100 rounded-lg font-medium transition-all duration-200 text-white dark:text-slate-900 shadow-lg shadow-slate-900/10 dark:shadow-none"
            >
              Get Started
            </a>
            <a
              href="https://github.com/kampouse/mpp-near"
              target="_blank"
              rel="noopener noreferrer"
              className="px-8 py-3.5 border border-slate-300 dark:border-slate-700 hover:border-slate-400 dark:hover:border-slate-600 rounded-lg font-medium transition-all duration-200 text-slate-700 dark:text-slate-300 flex items-center gap-2"
            >
              <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                <path fillRule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" clipRule="evenodd" />
              </svg>
              GitHub
            </a>
          </div>
        </div>

        <div className="grid md:grid-cols-2 gap-6">
          <div className="bg-white dark:bg-slate-800/30 border border-slate-200 dark:border-slate-700 rounded-xl p-8 hover:border-[#00C08B]/30 dark:hover:border-[#00C08B]/20 transition-all duration-200 shadow-sm dark:shadow-none">
            <div className="flex items-center gap-3 mb-6">
              <div className="w-10 h-10 rounded-lg flex items-center justify-center" style={{backgroundColor: 'rgba(0, 192, 139, 0.1)'}}>
                <svg className="w-5 h-5" style={{color: '#00C08B'}} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2m-2-4h.01M17 16h.01" />
                </svg>
              </div>
              <div>
                <h3 className="text-lg font-semibold text-slate-900 dark:text-slate-100">Rust API</h3>
                <p className="text-sm text-slate-500">For Service Providers</p>
              </div>
            </div>
            <p className="text-slate-600 dark:text-slate-400 mb-6 leading-relaxed text-sm">
              Build payment-gated APIs with type-safe Rust primitives. Accept NEAR, USDC, and 100+ tokens automatically.
            </p>
            <div className="space-y-3">
              <div className="flex items-center gap-3 text-sm">
                <svg className="w-4 h-4 flex-shrink-0" style={{color: '#00C08B'}} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                </svg>
                <span className="text-slate-700 dark:text-slate-300">Monetize APIs automatically</span>
              </div>
              <div className="flex items-center gap-3 text-sm">
                <svg className="w-4 h-4 flex-shrink-0" style={{color: '#00C08B'}} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                </svg>
                <span className="text-slate-700 dark:text-slate-300">Stateless verification</span>
              </div>
              <div className="flex items-center gap-3 text-sm">
                <svg className="w-4 h-4 flex-shrink-0" style={{color: '#00C08B'}} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                </svg>
                <span className="text-slate-700 dark:text-slate-300">Standards-compliant</span>
              </div>
            </div>
            <div className="mt-6 pt-6 border-t border-slate-200 dark:border-slate-700">
              <a href="/docs/api/overview" className="text-sm font-medium inline-flex items-center gap-1" style={{color: '#00C08B'}}>
                Learn more
                <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
                </svg>
              </a>
            </div>
          </div>

          <div className="bg-white dark:bg-slate-800/30 border border-slate-200 dark:border-slate-700 rounded-xl p-8 hover:border-[#6366F1]/30 dark:hover:border-[#6366F1]/20 transition-all duration-200 shadow-sm dark:shadow-none">
            <div className="flex items-center gap-3 mb-6">
              <div className="w-10 h-10 rounded-lg flex items-center justify-center" style={{backgroundColor: 'rgba(99, 102, 241, 0.1)'}}>
                <svg className="w-5 h-5" style={{color: '#6366F1'}} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
                </svg>
              </div>
              <div>
                <h3 className="text-lg font-semibold text-slate-900 dark:text-slate-100">CLI Tools</h3>
                <p className="text-sm text-slate-500">For Agents & Clients</p>
              </div>
            </div>
            <p className="text-slate-600 dark:text-slate-400 mb-6 leading-relaxed text-sm">
              Empower AI agents, scripts, and automated systems to pay for services, swap tokens, and transact gaslessly. Build from source for full features.
            </p>
            <div className="space-y-3">
              <div className="flex items-center gap-3 text-sm">
                <svg className="w-4 h-4 flex-shrink-0" style={{color: '#00C08B'}} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                </svg>
                <span className="text-slate-700 dark:text-slate-300">Gasless payments</span>
              </div>
              <div className="flex items-center gap-3 text-sm">
                <svg className="w-4 h-4 flex-shrink-0" style={{color: '#00C08B'}} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                </svg>
                <span className="text-slate-700 dark:text-slate-300">Agent-to-agent payments</span>
              </div>
              <div className="flex items-center gap-3 text-sm">
                <svg className="w-4 h-4 flex-shrink-0" style={{color: '#00C08B'}} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                </svg>
                <span className="text-slate-700 dark:text-slate-300">Cross-chain swaps</span>
              </div>
            </div>
            <div className="mt-6 pt-6 border-t border-slate-200 dark:border-slate-700">
              <a href="/docs/cli/overview" className="text-sm font-medium inline-flex items-center gap-1" style={{color: '#6366F1'}}>
                Learn more
                <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
                </svg>
              </a>
            </div>
          </div>
        </div>
      </div>
    </section>
  )
}
