export default function PixelArtLogo() {
  return (
    <div className="flex items-center justify-center gap-4 md:gap-6 mb-4">
      {/* MPP Pixel Art */}
      <div className="relative">
        <div
          className="pixel-text"
          style={{
            fontFamily: 'Inter, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif',
            fontSize: 'clamp(48px, 10vw, 96px)',
            fontWeight: '800',
            letterSpacing: '-0.02em',
            textShadow: `
              3px 3px 0 #00C08B,
              -1px -1px 0 #00C08B,
              1px -1px 0 #00C08B,
              -1px 1px 0 #00C08B,
              5px 5px 8px rgba(0, 192, 139, 0.25)
            `,
            imageRendering: 'pixelated',
            WebkitFontSmoothing: 'antialiased',
            MozOsxFontSmoothing: 'grayscale',
          }}
        >
          <span className="text-slate-900 dark:text-white">MPP</span>
        </div>
      </div>

      {/* "on" text */}
      <div
        className="text-xl md:text-2xl font-semibold text-slate-500 dark:text-slate-400"
        style={{
          fontFamily: 'Inter, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif',
          letterSpacing: '-0.01em',
        }}
      >
        on
      </div>

      {/* NEAR Pixel Art */}
      <div className="relative">
        <div
          className="pixel-text"
          style={{
            fontFamily: 'Inter, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif',
            fontSize: 'clamp(48px, 10vw, 96px)',
            fontWeight: '800',
            letterSpacing: '-0.02em',
            textShadow: `
              3px 3px 0 #00C08B,
              -1px -1px 0 #00C08B,
              1px -1px 0 #00C08B,
              -1px 1px 0 #00C08B,
              5px 5px 8px rgba(0, 192, 139, 0.25)
            `,
            imageRendering: 'pixelated',
            WebkitFontSmoothing: 'antialiased',
            MozOsxFontSmoothing: 'grayscale',
          }}
        >
          <span className="text-slate-900 dark:text-white">NEAR</span>
        </div>
      </div>
    </div>
  )
}
