import fs from 'fs/promises'
import path from 'path'

async function getSkillContent() {
  const skillPath = path.join(process.cwd(), '../SKILL.md')

  try {
    const content = await fs.readFile(skillPath, 'utf-8')
    return content
  } catch (error) {
    return '# Skill file not found\n\nThe SKILL.md file could not be loaded.'
  }
}

export default async function SkillPage() {
  const content = await getSkillContent()

  return (
    <div className="min-h-screen bg-white dark:bg-slate-950">
      <div className="max-w-4xl mx-auto px-6 py-12">
        <div className="mb-8">
          <h1 className="text-4xl font-bold mb-4 text-slate-900 dark:text-slate-50">MPP-NEAR Skill</h1>
          <p className="text-slate-600 dark:text-slate-400">
            NEAR payment CLI for Machine Payments Protocol (MPP)
          </p>
        </div>

        <div className="bg-white dark:bg-slate-800/50 border border-slate-200 dark:border-slate-700 rounded-xl p-8 overflow-x-auto">
          <pre className="whitespace-pre-wrap text-sm text-slate-700 dark:text-slate-300 font-mono leading-relaxed">
            {content}
          </pre>
        </div>

        <div className="mt-8">
          <a
            href="/"
            className="text-sm" style={{color: '#00C08B'}}
          >
            ← Back to Home
          </a>
        </div>
      </div>
    </div>
  )
}
