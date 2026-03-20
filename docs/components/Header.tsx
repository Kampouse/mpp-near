export default function Header() {
  return (
    <header className="fixed top-0 w-full bg-near-dark/90 backdrop-blur-sm border-b border-gray-800 z-50">
      <div className="max-w-6xl mx-auto px-6 py-4 flex items-center justify-between">
        <div className="flex items-center gap-3">
          <div className="w-8 h-8 bg-gradient-to-br from-blue-500 to-purple-600 rounded-lg" />
          <span className="font-bold text-xl">MPP-NEAR</span>
        </div>
        <nav className="hidden md:flex items-center gap-8 text-sm text-gray-300">
          <a href="#what-is-mpp" className="hover:text-white transition-colors">What is MPP</a>
          <a href="#why-use" className="hover:text-white transition-colors">Why Use It</a>
          <a href="#features" className="hover:text-white transition-colors">Features</a>
          <a href="#how-it-works" className="hover:text-white transition-colors">How It Works</a>
          <a href="#comparison" className="hover:text-white transition-colors">Comparison</a>
        </nav>
        <a 
          href="https://github.com/kampouse/mpp-near"
          target="_blank"
          rel="noopener noreferrer"
          className="px-4 py-2 bg-white/10 hover:bg-white/20 rounded-lg text-sm font-medium transition-colors"
        >
          GitHub
        </a>
      </div>
    </header>
  )
}
