export default function Hero() {
  return (
    <section className="pt-32 pb-20 px-6">
      <div className="max-w-6xl mx-auto text-center">
        <div className="inline-flex items-center gap-2 px-4 py-2 bg-blue-500/10 border border-blue-500/20 rounded-full text-blue-400 text-sm mb-8">
          <span className="w-2 h-2 bg-blue-400 rounded-full animate-pulse" />
          Open Protocol • NEAR Blockchain • Gasless Payments
        </div>
        
        <h1 className="text-5xl md:text-7xl font-bold mb-6 text-balance">
          Machine Payments Protocol
          <span className="block text-transparent bg-clip-text bg-gradient-to-r from-blue-400 to-purple-500">
            for NEAR
          </span>
        </h1>
        
        <p className="text-xl text-gray-400 max-w-3xl mx-auto mb-12">
          An open implementation of the Machine Payments Protocol (MPP) enabling 
          HTTP 402 machine-to-machine payments on the NEAR blockchain with gasless transactions.
        </p>
        
        <div className="flex flex-col sm:flex-row items-center justify-center gap-4">
          <a 
            href="#what-is-mpp"
            className="px-8 py-3 bg-white text-black rounded-lg font-medium hover:bg-gray-100 transition-colors"
          >
            Learn More
          </a>
          <a 
            href="https://github.com/kampouse/mpp-near"
            target="_blank"
            rel="noopener noreferrer"
            className="px-8 py-3 border border-gray-600 rounded-lg font-medium hover:bg-white/5 transition-colors"
          >
            View on GitHub
          </a>
        </div>
      </div>
    </section>
  )
}
