"use client";

import { useState } from "react";

export default function DocsLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);

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
      {/* Mobile Header */}
      <div className="lg:hidden fixed top-0 left-0 right-0 bg-white dark:bg-slate-950 border-b border-slate-200 dark:border-slate-800 z-50">
        <div className="flex items-center justify-between px-4 py-4">
          <a
            href="/docs"
            className="text-lg font-bold text-slate-900 dark:text-slate-100"
          >
            MPP-NEAR Docs
          </a>
          <button
            onClick={() => setIsMobileMenuOpen(!isMobileMenuOpen)}
            className="p-2 -mr-2 text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-100 transition-colors"
            aria-label="Toggle menu"
          >
            <svg
              className="w-6 h-6"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              {isMobileMenuOpen ? (
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M6 18L18 6M6 6l12 12"
                />
              ) : (
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M4 6h16M4 12h16M4 18h16"
                />
              )}
            </svg>
          </button>
        </div>
      </div>

      {/* Mobile Backdrop */}
      {isMobileMenuOpen && (
        <div
          className="lg:hidden fixed inset-0 bg-black/50 z-30"
          onClick={() => setIsMobileMenuOpen(false)}
        />
      )}

      <div className="flex">
        {/* Sidebar */}
        <aside
          className={`
            fixed lg:static inset-y-0 left-0 z-40
            w-72 min-h-screen
            bg-slate-50 dark:bg-slate-900
            border-r border-slate-200 dark:border-slate-800
            transform transition-transform duration-300 ease-in-out
            ${isMobileMenuOpen ? "translate-x-0" : "-translate-x-full lg:translate-x-0"}
            lg:translate-x-0
            pt-16 lg:pt-6
            px-6
            overflow-y-auto
          `}
        >
          {/* Desktop Header (hidden on mobile) */}
          <div className="hidden lg:block mb-8">
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
                        onClick={() => setIsMobileMenuOpen(false)}
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
        <main className="flex-1 max-w-4xl px-4 lg:px-8 py-20 lg:py-12 min-w-0">
          {children}
        </main>
      </div>
    </div>
  );
}
