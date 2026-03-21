export default function WhatIsMPP() {
  return (
    <section id="what-is-mpp" className="py-24 px-6 bg-slate-50 dark:bg-slate-900/30">
      <div className="max-w-6xl mx-auto">
        <div className="grid md:grid-cols-2 gap-16 items-start">
          <div>
            <h2 className="text-3xl md:text-4xl font-bold mb-6 text-slate-900 dark:text-slate-50">
              What is MPP?
            </h2>
            <p className="text-slate-600 dark:text-slate-400 mb-6 leading-relaxed">
              The Machine Payments Protocol (MPP) is an{" "}
              <a href="https://mpp.dev" target="_blank" rel="noopener noreferrer" className="hover:underline font-medium" style={{color: '#00C08B', textDecorationColor: 'rgba(0, 192, 139, 0.3)'}}>
                open standard
              </a>{" "}
              for machine-to-machine payments that standardizes HTTP 402 "Payment Required" responses.
            </p>
            <p className="text-slate-600 dark:text-slate-400 mb-8 leading-relaxed">
              MPP-NEAR implements this protocol for the NEAR blockchain, allowing services to accept NEAR and NEP-141 tokens (USDC, USDT) as payment for API calls in a single HTTP request.
            </p>

            <div className="space-y-6">
              <div className="flex gap-4">
                <div className="flex-shrink-0 w-8 h-8 rounded-lg flex items-center justify-center font-semibold text-sm" style={{backgroundColor: 'rgba(0, 192, 139, 0.1)', color: '#00C08B'}}>
                  1
                </div>
                <div>
                  <h4 className="font-semibold text-slate-900 dark:text-slate-100 mb-1">Request Protected Resource</h4>
                  <p className="text-slate-600 dark:text-slate-400 text-sm">
                    Client accesses API endpoint requiring payment
                  </p>
                </div>
              </div>

              <div className="flex gap-4">
                <div className="flex-shrink-0 w-8 h-8 rounded-lg flex items-center justify-center font-semibold text-sm" style={{backgroundColor: 'rgba(99, 102, 241, 0.1)', color: '#6366F1'}}>
                  2
                </div>
                <div>
                  <h4 className="font-semibold text-slate-900 dark:text-slate-100 mb-1">Receive HTTP 402 Challenge</h4>
                  <p className="text-slate-600 dark:text-slate-400 text-sm">
                    Server returns payment requirements in WWW-Authenticate header
                  </p>
                </div>
              </div>

              <div className="flex gap-4">
                <div className="flex-shrink-0 w-8 h-8 rounded-lg flex items-center justify-center font-semibold text-sm" style={{backgroundColor: 'rgba(249, 115, 22, 0.1)', color: '#F97316'}}>
                  3
                </div>
                <div>
                  <h4 className="font-semibold text-slate-900 dark:text-slate-100 mb-1">Pay & Submit Credential</h4>
                  <p className="text-slate-600 dark:text-slate-400 text-sm">
                    Client completes payment and retries with proof in Authorization header
                  </p>
                </div>
              </div>

              <div className="flex gap-4">
                <div className="flex-shrink-0 w-8 h-8 rounded-lg flex items-center justify-center font-semibold text-sm" style={{backgroundColor: 'rgba(34, 197, 94, 0.1)', color: '#22C55E'}}>
                  4
                </div>
                <div>
                  <h4 className="font-semibold text-slate-900 dark:text-slate-100 mb-1">Access Granted</h4>
                  <p className="text-slate-600 dark:text-slate-400 text-sm">
                    Server verifies payment and returns requested resource
                  </p>
                </div>
              </div>
            </div>
          </div>

          <div className="bg-white dark:bg-slate-800/50 border border-slate-200 dark:border-slate-700 rounded-xl overflow-hidden shadow-sm dark:shadow-none">
            <div className="px-6 py-4 border-b border-slate-200 dark:border-slate-700 bg-slate-50 dark:bg-slate-800/30">
              <div className="text-sm font-mono text-slate-500 dark:text-slate-400">HTTP 402 Payment Flow</div>
            </div>
            <div className="p-6 space-y-4 font-mono text-sm bg-slate-50/50 dark:bg-slate-900/20">
              <div className="flex items-start gap-3">
                <span className="text-slate-400 dark:text-slate-500 mt-0.5 select-none">$</span>
                <div className="text-slate-700 dark:text-slate-300">curl https://api.example.com/generate</div>
              </div>

              <div className="flex items-start gap-3">
                <span className="text-slate-400 dark:text-slate-500 mt-0.5 select-none">←</span>
                <div>
                  <div style={{color: '#6366F1'}}>HTTP/1.1 402 Payment Required</div>
                  <div className="text-slate-500 dark:text-slate-500 text-xs break-all mt-1">WWW-Authenticate: Payment realm="api.example.com", method="near-intents", amount="0.001"</div>
                </div>
              </div>

              <div className="flex items-start gap-3">
                <span className="text-slate-400 dark:text-slate-500 mt-0.5 select-none">$</span>
                <div className="text-slate-700 dark:text-slate-300 text-xs">curl https://api.example.com/generate \ -H "Authorization: Payment eyJjaGFs..."</div>
              </div>

              <div className="flex items-start gap-3">
                <span className="text-slate-400 dark:text-slate-500 mt-0.5 select-none">←</span>
                <div>
                  <div style={{color: '#00C08B'}}>HTTP/1.1 200 OK</div>
                  <div className="text-slate-500 dark:text-slate-500 text-xs mt-1">Payment-Receipt: Payment id="xyz789", amount="0.001"</div>
                </div>
              </div>

              <div className="flex items-start gap-3">
                <span className="text-slate-400 dark:text-slate-500 mt-0.5 select-none">{"{}"}</span>
                <div className="text-slate-500 dark:text-slate-500 text-xs">"data": "Generated content..."</div>
              </div>
            </div>
          </div>
        </div>

        <div className="mt-16 grid md:grid-cols-3 gap-6">
          <div className="text-center p-6 bg-white dark:bg-slate-800/30 border border-slate-200 dark:border-slate-700 rounded-xl">
            <div className="text-3xl font-bold text-slate-900 dark:text-slate-50 mb-2">HTTP 402</div>
            <div className="text-sm text-slate-600 dark:text-slate-400">Standard protocol</div>
          </div>
          <div className="text-center p-6 bg-white dark:bg-slate-800/30 border border-slate-200 dark:border-slate-700 rounded-xl">
            <div className="text-3xl font-bold text-slate-900 dark:text-slate-50 mb-2">$0.001</div>
            <div className="text-sm text-slate-600 dark:text-slate-400">Per transaction</div>
          </div>
          <div className="text-center p-6 bg-white dark:bg-slate-800/30 border border-slate-200 dark:border-slate-700 rounded-xl">
            <div className="text-3xl font-bold text-slate-900 dark:text-slate-50 mb-2">20+</div>
            <div className="text-sm text-slate-600 dark:text-slate-400">Blockchains supported</div>
          </div>
        </div>
      </div>
    </section>
  )
}
