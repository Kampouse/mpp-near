'use client'

import { useState } from 'react'

export default function CodeExample() {
  const [activeTab, setActiveTab] = useState<'client' | 'server' | 'swap'>('client')

  const examples = {
    client: {
      title: "Client - Handle 402 Payments",
      code: `use mpp_near::client::{IntentsProvider};
use reqwest_middleware::ClientBuilder;

let provider = IntentsProvider::new("wk_...".to_string());

let client = ClientBuilder::new(reqwest::Client::new())
    .with(PaymentMiddleware::new(provider))
    .build();

// Automatically handles 402 responses
let resp = client
    .get("https://api.example.com/paid")
    .send()
    .await?;`
    },
    server: {
      title: "Server - Accept Payments",
      code: `use mpp_near::server::{NearVerifier, NearPayment};

let verifier = NearVerifier::new(VerifierConfig {
    recipient_account: "merchant.near".parse()?,
    min_amount: NearAmount::from_near(1),
    ..Default::default()
})?;

// Extract payment in handler
async fn handler(payment: NearPayment) -> String {
    format!("Paid by: {}", payment.payer())
}`
    },
    swap: {
      title: "Swap Tokens (Gasless)",
      code: `// Swap 1 NEAR to USDC - no gas required
let result = provider.swap(
    "nep141:wrap.near",        // wNEAR
    "nep141:usdc.omft.near",   // USDC
    NearAmount::from_near(1),
    None,  // No slippage protection
).await?;

println!("Got {} USDC", result.amount_out);`
    }
  }

  return (
    <section className="py-20 px-6 bg-near-gray/50">
      <div className="max-w-6xl mx-auto">
        <div className="text-center mb-12">
          <h2 className="text-3xl md:text-4xl font-bold mb-4">
            Code Examples
          </h2>
          <p className="text-gray-400">
            Rust SDK for integrating NEAR payments
          </p>
        </div>
        
        <div className="bg-near-dark rounded-xl border border-gray-800 overflow-hidden">
          <div className="flex border-b border-gray-800">
            {Object.entries(examples).map(([key, value]) => (
              <button
                key={key}
                onClick={() => setActiveTab(key as any)}
                className={`px-6 py-3 text-sm font-medium transition-colors ${
                  activeTab === key 
                    ? 'text-white bg-near-gray/50 border-b-2 border-blue-500' 
                    : 'text-gray-400 hover:text-white'
                }`}
              >
                {value.title}
              </button>
            ))}
          </div>
          <pre className="p-6 text-sm overflow-x-auto">
            <code className="text-gray-300 font-mono whitespace-pre">
              {examples[activeTab].code}
            </code>
          </pre>
        </div>
      </div>
    </section>
  )
}
