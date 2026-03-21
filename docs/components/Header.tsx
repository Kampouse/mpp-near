"use client";

import { useState } from "react";

export default function Header() {
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);
  return (
    <header className="fixed top-0 w-full bg-white/95 dark:bg-slate-900/95 backdrop-blur-sm border-b border-slate-200 dark:border-slate-800 z-50">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 py-4">
        <div className="flex items-center justify-between">
          {/* Logo */}
          <div className="flex items-center gap-3">
            <div>
              <span className="font-bold text-lg text-slate-900 dark:text-slate-50">
                MPP-NEAR
              </span>
              <div className="text-xs text-slate-500 dark:text-slate-400 -mt-1">
                Machine Payments Protocol
              </div>
            </div>
          </div>

          {/* Desktop Navigation */}
          <nav className="hidden lg:flex items-center gap-8 text-sm text-slate-600 dark:text-slate-400">
            <a
              href="#what-is-mpp"
              className="hover:text-slate-900 dark:hover:text-slate-100 transition-colors"
            >
              Overview
            </a>
            <a
              href="#why-use"
              className="hover:text-slate-900 dark:hover:text-slate-100 transition-colors"
            >
              Why MPP
            </a>
            <a
              href="/docs/api/overview"
              className="hover:text-slate-900 dark:hover:text-slate-100 transition-colors"
            >
              Service API
            </a>
            <a
              href="/docs/cli/overview"
              className="hover:text-slate-900 dark:hover:text-slate-100 transition-colors"
            >
              Agent CLI
            </a>
            <a
              href="/docs"
              className="hover:text-slate-900 dark:hover:text-slate-100 transition-colors"
            >
              Documentation
            </a>
          </nav>

          {/* Mobile Menu Button */}
          <div className="lg:hidden">
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

          {/* CTA Buttons */}
          <div className="hidden lg:flex items-center gap-3">
            <a
              href="https://github.com/kampouse/mpp-near"
              target="_blank"
              rel="noopener noreferrer"
              className="p-2 text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-slate-100 transition-colors"
              aria-label="GitHub"
            >
              <svg
                className="w-5 h-5"
                fill="currentColor"
                viewBox="0 0 24 24"
                aria-hidden="true"
              >
                <path
                  fillRule="evenodd"
                  d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z"
                  clipRule="evenodd"
                />
              </svg>
            </a>
            <a
              href="/docs"
              className="px-4 py-2 bg-slate-900 dark:bg-slate-50 hover:bg-slate-800 dark:hover:bg-slate-100 rounded-lg text-sm font-medium transition-colors text-white dark:text-slate-900"
            >
              Get Started
            </a>
          </div>
        </div>
      </div>

      {/* Mobile Menu */}
      {isMobileMenuOpen && (
        <div className="lg:hidden fixed top-[60px] left-0 right-0 bg-white dark:bg-slate-900 border-b border-slate-200 dark:border-slate-800 z-40">
          <div className="max-w-7xl mx-auto px-4 py-4">
            <nav className="flex flex-col gap-4 text-sm text-slate-600 dark:text-slate-400">
              <a
                href="#what-is-mpp"
                className="py-3 px-4 hover:bg-slate-100 dark:hover:bg-slate-800 hover:text-slate-900 dark:hover:text-slate-100 rounded-lg transition-colors"
                onClick={() => setIsMobileMenuOpen(false)}
              >
                Overview
              </a>
              <a
                href="#why-use"
                className="py-3 px-4 hover:bg-slate-100 dark:hover:bg-slate-800 hover:text-slate-900 dark:hover:text-slate-100 rounded-lg transition-colors"
                onClick={() => setIsMobileMenuOpen(false)}
              >
                Why MPP
              </a>
              <a
                href="/docs/api/overview"
                className="py-3 px-4 hover:bg-slate-100 dark:hover:bg-slate-800 hover:text-slate-900 dark:hover:text-slate-100 rounded-lg transition-colors"
                onClick={() => setIsMobileMenuOpen(false)}
              >
                Service API
              </a>
              <a
                href="/docs/cli/overview"
                className="py-3 px-4 hover:bg-slate-100 dark:hover:bg-slate-800 hover:text-slate-900 dark:hover:text-slate-100 rounded-lg transition-colors"
                onClick={() => setIsMobileMenuOpen(false)}
              >
                Agent CLI
              </a>
              <a
                href="/docs"
                className="py-3 px-4 hover:bg-slate-100 dark:hover:bg-slate-800 hover:text-slate-900 dark:hover:text-slate-100 rounded-lg transition-colors"
                onClick={() => setIsMobileMenuOpen(false)}
              >
                Documentation
              </a>
            </nav>
          </div>
        </div>
      )}
    </header>
  );
}
