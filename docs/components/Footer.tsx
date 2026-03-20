export default function Footer() {
  return (
    <footer className="py-12 px-6 border-t border-gray-800">
      <div className="max-w-6xl mx-auto">
        <div className="grid md:grid-cols-4 gap-8 mb-8">
          <div>
            <div className="flex items-center gap-2 mb-4">
              <div className="w-6 h-6 bg-gradient-to-br from-blue-500 to-purple-600 rounded" />
              <span className="font-bold">MPP-NEAR</span>
            </div>
            <p className="text-sm text-gray-400">
              Machine Payments Protocol implementation for NEAR blockchain.
            </p>
          </div>
          
          <div>
            <h4 className="font-semibold mb-4">Resources</h4>
            <ul className="space-y-2 text-sm text-gray-400">
              <li>
                <a href="https://mpp.dev" target="_blank" rel="noopener noreferrer" className="hover:text-white transition-colors">
                  MPP Specification
                </a>
              </li>
              <li>
                <a href="https://near.org" target="_blank" rel="noopener noreferrer" className="hover:text-white transition-colors">
                  NEAR Blockchain
                </a>
              </li>
              <li>
                <a href="https://outlayer.fastnear.com" target="_blank" rel="noopener noreferrer" className="hover:text-white transition-colors">
                  OutLayer Dashboard
                </a>
              </li>
            </ul>
          </div>
          
          <div>
            <h4 className="font-semibold mb-4">Documentation</h4>
            <ul className="space-y-2 text-sm text-gray-400">
              <li>
                <a href="https://github.com/kampouse/mpp-near" target="_blank" rel="noopener noreferrer" className="hover:text-white transition-colors">
                  GitHub Repository
                </a>
              </li>
              <li>
                <a href="https://github.com/kampouse/mpp-near/blob/main/README.md" target="_blank" rel="noopener noreferrer" className="hover:text-white transition-colors">
                  README
                </a>
              </li>
              <li>
                <a href="https://github.com/kampouse/mpp-near/issues" target="_blank" rel="noopener noreferrer" className="hover:text-white transition-colors">
                  Report Issues
                </a>
              </li>
            </ul>
          </div>
          
          <div>
            <h4 className="font-semibold mb-4">License</h4>
            <p className="text-sm text-gray-400 mb-2">
              Dual-licensed under MIT or Apache-2.0
            </p>
            <p className="text-xs text-gray-500">
              Forked from tempoxyz/mpp-rs
            </p>
          </div>
        </div>
        
        <div className="pt-8 border-t border-gray-800 text-center text-sm text-gray-500">
          <p>
            Open source implementation • Not affiliated with NEAR Foundation or MPP authors
          </p>
        </div>
      </div>
    </footer>
  )
}
