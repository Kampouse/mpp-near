export default function DocsLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const navItems = [
    {
      title: "Getting Started",
      items: [
        { href: "/docs", label: "Documentation Home" },
        { href: "/docs/installation", label: "Installation" },
        { href: "/docs/quickstart", label: "Quick Start" },
      ],
    },
    {
      title: "For Service Providers",
      items: [
        { href: "/docs/api/overview", label: "API Overview" },
        { href: "/docs/api/overview#challenge", label: "Creating Challenges" },
        { href: "/docs/api/overview#credential", label: "Verifying Payments" },
      ],
    },
    {
      title: "For Agents & Clients",
      items: [
        { href: "/docs/cli/overview", label: "CLI Reference" },
        { href: "/skill", label: "AI Skill File" },
      ],
    },
    {
      title: "Integration",
      items: [
        { href: "/docs/outlayer", label: "OutLayer" },
        { href: "/docs/faq", label: "FAQ" },
      ],
    },
  ];

  return (
    <div className="min-h-screen bg-white dark:bg-slate-950">
      <div className="flex">
        {/* Sidebar */}
        <aside className="w-64 min-h-screen bg-slate-50 dark:bg-slate-900 border-r border-slate-200 dark:border-slate-800 p-6 sticky top-0 overflow-y-auto">
          <div className="mb-8">
            <a
              href="/docs"
              className="text-xl font-bold text-slate-900 dark:text-slate-100"
            >
              MPP-NEAR Docs
            </a>
          </div>
          <nav className="space-y-6">
            {navItems.map((section) => (
              <div key={section.title}>
                <h3 className="text-xs font-semibold text-slate-500 uppercase tracking-wider mb-3">
                  {section.title}
                </h3>
                <ul className="space-y-2">
                  {section.items.map((item) => (
                    <li key={item.href}>
                      <a
                        href={item.href}
                        className="text-sm text-slate-700 dark:text-slate-300 hover:text-slate-900 dark:hover:text-slate-100 block py-1"
                      >
                        {item.label}
                      </a>
                    </li>
                  ))}
                </ul>
              </div>
            ))}
          </nav>
        </aside>

        {/* Main content */}
        <main className="flex-1 max-w-4xl px-8 py-12">{children}</main>
      </div>
    </div>
  );
}
