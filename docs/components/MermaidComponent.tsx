'use client'

import { useEffect, useState } from 'react'
import mermaid from 'mermaid'

interface MermaidProps {
  chart: string
}

// Initialize mermaid once
let mermaidInitialized = false
if (typeof window !== 'undefined' && !mermaidInitialized) {
  mermaid.initialize({
    startOnLoad: false,
    theme: 'default',
    securityLevel: 'loose',
    logLevel: 'error',
    suppressErrorRendering: true,
  })
  mermaidInitialized = true
}

export function MermaidComponent({ chart }: MermaidProps) {
  const [error, setError] = useState<string | null>(null)
  const [loading, setLoading] = useState(true)
  const [svgContent, setSvgContent] = useState<string>('')

  useEffect(() => {
    let isMounted = true

    const renderDiagram = async () => {
      try {
        setLoading(true)
        setError(null)

        // Generate unique ID for this diagram
        const id = `mermaid-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`

        // Render using mermaid's render API
        const { svg } = await mermaid.render(id, chart)

        if (isMounted) {
          setSvgContent(svg)
          setLoading(false)
        }
      } catch (err) {
        console.error('Mermaid rendering error:', err)
        if (isMounted) {
          setError(err instanceof Error ? err.message : 'Failed to render diagram')
          setLoading(false)
          setSvgContent('')
        }
      }
    }

    renderDiagram()

    // Cleanup function
    return () => {
      isMounted = false
    }
  }, [chart])

  if (error) {
    return (
      <div className="mermaid-error flex justify-center items-center my-4 p-4 bg-red-50 rounded-lg border border-red-200">
        <div className="text-red-600 text-sm text-center">
          <p className="font-medium">Failed to render diagram</p>
          <p className="text-xs mt-1 text-red-500">{error}</p>
        </div>
      </div>
    )
  }

  return (
    <div className="mermaid-wrapper my-6 flex justify-center">
      {loading ? (
        <div className="mermaid-loading flex items-center justify-center p-8 bg-gray-50 rounded-lg border border-gray-200">
          <div className="text-gray-400 text-sm">Loading diagram...</div>
        </div>
      ) : (
        <div
          className="mermaid-diagram"
          dangerouslySetInnerHTML={{ __html: svgContent }}
          style={{ maxWidth: '100%', overflow: 'auto' }}
        />
      )}
    </div>
  )
}
