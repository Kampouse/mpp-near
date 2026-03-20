export default function WhatIsMPP() {
  return (
    <section id="what-is-mpp" className="py-20 px-6 bg-near-gray/50">
      <div className="max-w-6xl mx-auto">
        <div className="grid md:grid-cols-2 gap-12 items-center">
          <div>
            <h2 className="text-3xl md:text-4xl font-bold mb-6">
              What is MPP?
            </h2>
            <p className="text-gray-400 mb-6 leading-relaxed">
              The Machine Payments Protocol (MPP) is an open standard for machine-to-machine 
              payments. It standardizes HTTP 402 "Payment Required" responses, enabling 
              autonomous systems to pay for API access, services, and resources automatically.
            </p>
            <p className="text-gray-400 mb-6 leading-relaxed">
              <strong className="text-white">MPP-NEAR</strong> implements this protocol for the 
              NEAR blockchain, allowing services to accept NEAR and NEP-141 tokens (like USDC, USDT) 
              as payment for API calls in a single HTTP request.
            </p>
            <div className="flex items-center gap-4 text-sm">
              <a 
                href="https://mpp.dev" 
                target="_blank" 
                rel="noopener noreferrer"
                className="text-blue-400 hover:text-blue-300 transition-colors"
              >
                MPP Specification →
              </a>
              <span className="text-gray-600">|</span>
              <a 
                href="https://near.org" 
                target="_blank" 
                rel="noopener noreferrer"
                className="text-blue-400 hover:text-blue-300 transition-colors"
              >
                NEAR Blockchain →
              </a>
            </div>
          </div>
          
          <div className="bg-near-dark rounded-xl p-6 border border-gray-800">
            <div className="text-sm text-gray-500 mb-4">HTTP 402 Payment Flow</div>
            <div className="space-y-4 font-mono text-sm">
              <div className="flex items-start gap-3">
                <span className="text-gray-500 mt-0.5">1.</span>
                <div>
                  <div className="text-gray-300">Client → Server</div>
                  <div className="text-gray-500">GET /api/resource</div>
                </div>
              </div>
              <div className="flex items-start gap-3">
                <span className="text-gray-500 mt-0.5">2.</span>
                <div>
                  <div className="text-orange-400">Server → Client (402)</div>
                  <div className="text-gray-500">{"{ amount: 1 NEAR, recipient: merchant.near }"}</div>
                </div>
              </div>
              <div className="flex items-start gap-3">
                <span className="text-gray-500 mt-0.5">3.</span>
                <div>
                  <div className="text-gray-300">Client pays on-chain</div>
                  <div className="text-gray-500">Transfer 1 NEAR to merchant.near</div>
                </div>
              </div>
              <div className="flex items-start gap-3">
                <span className="text-gray-500 mt-0.5">4.</span>
                <div>
                  <div className="text-gray-300">Client → Server</div>
                  <div className="text-gray-500">Authorization: Payment tx_hash</div>
                </div>
              </div>
              <div className="flex items-start gap-3">
                <span className="text-gray-500 mt-0.5">5.</span>
                <div>
                  <div className="text-green-400">Server → Client (200)</div>
                  <div className="text-gray-500">Resource access granted</div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  )
}
