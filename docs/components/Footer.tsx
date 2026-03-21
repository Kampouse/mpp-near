export default function Footer() {
  return (
    <footer className="py-16 px-6 border-t border-slate-200 dark:border-slate-800">
      <div className="max-w-6xl mx-auto">
        <div className="grid md:grid-cols-4 gap-12 mb-12">
          <div className="md:col-span-1">
            <p className="text-sm text-slate-600 dark:text-slate-400 leading-relaxed mb-6">
              Open implementation of the Machine Payments Protocol for NEAR blockchain.
            </p>
            <div className="flex gap-4">
              <a
                href="https://github.com/kampouse/mpp-near"
                target="_blank"
                rel="noopener noreferrer"
                className="text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-100 transition-colors"
                aria-label="GitHub"
              >
                <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                  <path fillRule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" clipRule="evenodd" />
                </svg>
              </a>
            </div>
          </div>

          <div>
            <h4 className="font-semibold mb-4 text-slate-900 dark:text-slate-50">Resources</h4>
            <ul className="space-y-3 text-sm text-slate-600 dark:text-slate-400">
              <li>
                <a href="https://mpp.dev" target="_blank" rel="noopener noreferrer" className="hover:text-slate-900 dark:hover:text-slate-100 transition-colors">
                  MPP Specification
                </a>
              </li>
              <li>
                <a href="https://near.org" target="_blank" rel="noopener noreferrer" className="hover:text-slate-900 dark:hover:text-slate-100 transition-colors">
                  NEAR Blockchain
                </a>
              </li>
              <li>
                <a href="https://outlayer.fastnear.com" target="_blank" rel="noopener noreferrer" className="hover:text-slate-900 dark:hover:text-slate-100 transition-colors font-medium" style={{color: '#00C08B'}}>
                  OutLayer (Gasless Custody)
                </a>
              </li>
              <li>
                <a href="https://paymentauth.org" target="_blank" rel="noopener noreferrer" className="hover:text-slate-900 dark:hover:text-slate-100 transition-colors">
                  PaymentAuth.org
                </a>
              </li>
            </ul>
          </div>

          <div>
            <h4 className="font-semibold mb-4 text-slate-900 dark:text-slate-50">Documentation</h4>
            <ul className="space-y-3 text-sm text-slate-600 dark:text-slate-400">
              <li>
                <a href="/docs" className="hover:text-slate-900 dark:hover:text-slate-100 transition-colors">
                  API Documentation
                </a>
              </li>
              <li>
                <a href="/docs" className="hover:text-slate-900 dark:hover:text-slate-100 transition-colors">
                  CLI Reference
                </a>
              </li>
              <li>
                <a href="#api" className="hover:text-slate-900 dark:hover:text-slate-100 transition-colors">
                  Quick Start Guide
                </a>
              </li>
              <li>
                <a href="https://github.com/kampouse/mpp-near/issues" target="_blank" rel="noopener noreferrer" className="hover:text-slate-900 dark:hover:text-slate-100 transition-colors">
                  Report Issues
                </a>
              </li>
            </ul>
          </div>

          <div>
            <h4 className="font-semibold mb-4 text-slate-900 dark:text-slate-50">Project</h4>
            <ul className="space-y-3 text-sm text-slate-600 dark:text-slate-400">
              <li>
                <a href="https://github.com/kampouse/mpp-near" target="_blank" rel="noopener noreferrer" className="hover:text-slate-900 dark:hover:text-slate-100 transition-colors">
                  GitHub Repository
                </a>
              </li>
              <li>
                <a href="https://github.com/kampouse/mpp-near/blob/main/LICENSE" target="_blank" rel="noopener noreferrer" className="hover:text-slate-900 dark:hover:text-slate-100 transition-colors">
                  License (MIT/Apache-2.0)
                </a>
              </li>
              <li>
                <div className="text-xs leading-relaxed">
                  Forked from{" "}
                  <a href="https://github.com/tempoxyz/mpp-rs" target="_blank" rel="noopener noreferrer" className="hover:underline" style={{color: '#00C08B'}}>
                    tempoxyz/mpp-rs
                  </a>
                </div>
              </li>
            </ul>
          </div>
        </div>

        <div className="pt-8 border-t border-slate-200 dark:border-slate-800">
          <div className="flex flex-col md:flex-row items-center justify-between gap-4 text-sm text-slate-500 dark:text-slate-500">
            <p>
              Open source implementation • Not affiliated with NEAR Foundation or MPP authors
            </p>
            <p>
              Built for NEAR on NEAR
            </p>
          </div>
        </div>
      </div>
    </footer>
  )
}
