import Header from '@/components/Header'
import Hero from '@/components/Hero'
import WhatIsMPP from '@/components/WhatIsMPP'
import WhyUseIt from '@/components/WhyUseIt'
import Features from '@/components/Features'
import HowItWorks from '@/components/HowItWorks'
import APISection from '@/components/APISection'
import CLISection from '@/components/CLISection'
import Comparison from '@/components/Comparison'
import Footer from '@/components/Footer'

export default function Home() {
  return (
    <main className="min-h-screen">
      <Header />
      <Hero />
      <WhatIsMPP />
      <WhyUseIt />
      <Features />
      <HowItWorks />
      <APISection />
      <CLISection />
      <Comparison />
      <Footer />
    </main>
  )
}
