import Link from 'next/link'

export default function DocsIndex() {
  const quickLinks = [
    {
      title: 'Quick Start',
      description: 'Get started in 5 minutes',
      href: '/docs/quickstart',
      icon: '🚀'
    },
    {
      title: 'API Documentation',
      description: 'Build payment-gated APIs',
      href: '/docs/api/overview',
      icon: '🔧'
    },
    {
      title: 'CLI Reference',
      description: 'For agents and automation',
      href: '/docs/cli/overview',
      icon: '⚡'
    },
    {
      title: 'Installation',
      description: 'Install from source',
      href: '/docs/installation',
      icon: '📦'
    }
  ]

  return (
    <div>
      <div className="mb-12">
        <h1 className="text-4xl font-bold mb-4 text-slate-900 dark:text-slate-50">MPP-NEAR Documentation</h1>
        <p className="text-xl text-slate-700 dark:text-slate-300">
          Complete guide to implementing Machine Payments Protocol on NEAR
        </p>
      </div>

      <div className="mb-16">
        <h2 className="text-2xl font-bold mb-6 text-slate-900 dark:text-slate-100">Quick Links</h2>
        <div className="grid md:grid-cols-2 gap-4">
          {quickLinks.map((link, idx) => (
            <Link
              key={idx}
              href={link.href}
              className="group flex items-start gap-4 p-6 rounded-lg border border-slate-200 dark:border-slate-800 hover:border-[#00C08B] dark:hover:border-[#00C08B] hover:bg-slate-50 dark:hover:bg-slate-900 transition-all"
            >
              <span className="text-2xl">{link.icon}</span>
              <div className="flex-1">
                <h3 className="font-semibold text-slate-900 dark:text-slate-100 group-hover:text-[#00C08B] dark:group-hover:text-[#00C08B] mb-1">
                  {link.title}
                  <span className="inline-block ml-2 transition-transform group-hover:translate-x-1">→</span>
                </h3>
                <p className="text-sm text-slate-700 dark:text-slate-300">{link.description}</p>
              </div>
            </Link>
          ))}
        </div>
      </div>

      <div className="grid md:grid-cols-2 gap-8 mb-16">
        <div>
          <h2 className="text-2xl font-bold mb-4 text-slate-900 dark:text-slate-100">For Service Providers</h2>
          <p className="text-slate-600 dark:text-slate-400 mb-6">
            Build payment-gated APIs with type-safe Rust primitives
          </p>
          <ul className="space-y-3">
            <li>
              <a href="/docs/api/overview#challenge" className="text-sm hover:text-[#00C08B] dark:hover:text-[#00C08B] flex items-center gap-2 text-slate-700 dark:text-slate-300">
                <span className="text-[#00C08B]">→</span>
                Creating payment challenges
              </a>
            </li>
            <li>
              <a href="/docs/api/overview#credential" className="text-sm hover:text-[#00C08B] dark:hover:text-[#00C08B] flex items-center gap-2 text-slate-700 dark:text-slate-300">
                <span className="text-[#00C08B]">→</span>
                Verifying payment credentials
              </a>
            </li>
            <li>
              <a href="/docs/api/overview#installation" className="text-sm hover:text-[#00C08B] dark:hover:text-[#00C08B] flex items-center gap-2 text-slate-700 dark:text-slate-300">
                <span className="text-[#00C08B]">→</span>
                Installation guide
              </a>
            </li>
          </ul>
        </div>

        <div>
          <h2 className="text-2xl font-bold mb-4 text-slate-900 dark:text-slate-100">For Agents & Clients</h2>
          <p className="text-slate-600 dark:text-slate-400 mb-6">
            CLI tool for AI agents and automated systems
          </p>
          <ul className="space-y-3">
            <li>
              <a href="/docs/cli/overview" className="text-sm hover:text-[#6366F1] dark:hover:text-[#6366F1] flex items-center gap-2 text-slate-700 dark:text-slate-300">
                <span className="text-[#6366F1]">→</span>
                CLI command reference
              </a>
            </li>
            <li>
              <a href="/skill" className="text-sm hover:text-[#6366F1] dark:hover:text-[#6366F1] flex items-center gap-2 text-slate-700 dark:text-slate-300">
                <span className="text-[#6366F1]">→</span>
                AI Skill file integration
              </a>
            </li>
            <li>
              <a href="/docs/outlayer" className="text-sm hover:text-[#6366F1] dark:hover:text-[#6366F1] flex items-center gap-2 text-slate-700 dark:text-slate-300">
                <span className="text-[#6366F1]">→</span>
                OutLayer gasless integration
              </a>
            </li>
          </ul>
        </div>
      </div>

      <div className="p-6 rounded-lg" style={{backgroundColor: 'rgba(0, 192, 139, 0.1)', borderColor: 'rgba(0, 192, 139, 0.2)'}}>
        <h3 className="font-semibold mb-2" style={{color: '#065F46'}}>Need Help?</h3>
        <p className="mb-4" style={{color: '#064E3B'}}>
          Check out our GitHub repository or join our community for support.
        </p>
        <div className="flex gap-4">
          <a
            href="https://github.com/kampouse/mpp-near"
            target="_blank"
            rel="noopener noreferrer"
            className="hover:underline font-medium" style={{color: '#047857'}}
          >
            GitHub Repository
          </a>
          <span style={{color: '#059669'}}>•</span>
          <a
            href="https://discord.gg/clawd"
            target="_blank"
            rel="noopener noreferrer"
            className="hover:underline font-medium" style={{color: '#047857'}}
          >
            Discord Community
          </a>
        </div>
      </div>
    </div>
  )
}
