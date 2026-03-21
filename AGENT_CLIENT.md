# Agent Client Documentation

The `AgentClient` is a seamless HTTP client for autonomous agents that automatically handles HTTP 402 Payment Required responses.

## Overview

Autonomous agents often need to access paid APIs, but handling payments manually is cumbersome. The `AgentClient` automates the entire payment flow:

1. Detects HTTP 402 responses
2. Parses payment challenges
3. Pays via OutLayer API (gasless)
4. Builds payment credentials
5. Retries requests with payment proof
6. Caches sessions to avoid re-payment

## Features

- ✅ **Auto-402 detection**: No manual handling needed
- ✅ **Gasless payments**: Uses OutLayer for feeless transactions
- ✅ **Budget controls**: Per-request and daily spending limits
- ✅ **Session caching**: Avoid re-paying for same resource
- ✅ **Receipt caching**: Reuse payment proofs
- ✅ **Type-safe**: Full Rust type safety

## Quick Start

### Rust Library

```rust
use mpp_near::client::{AgentClient, BudgetConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with budget limits
    let client = AgentClient::new("wk_your_api_key")
        .with_budget(BudgetConfig::new(0.10, 5.0)); // $0.10 per request, $5.00 per day
    
    // GET request - auto-handles 402 payment
    let data = client.get("https://paid-api.com/data").await?;
    
    // POST request - also auto-handles payment
    let result = client.post("https://api.example.com/submit", &serde_json::json!({
        "key": "value"
    })).await?;
    
    // Check spending
    println!("Spent today: ${:.4}", client.spent_today());
    println!("Remaining: ${:.4}", client.remaining_budget());
    
    Ok(())
}
```

### CLI Tool

```bash
# GET with auto-payment
mpp-near agent get --url https://api.example.com/data --max 0.10

# POST with auto-payment
mpp-near agent post --url https://api.example.com/submit --data '{"key":"value"}'

# Check budget status
mpp-near agent budget

# Test 402 flow (dry run)
mpp-near agent test --url https://api.example.com/data

# Clear payment cache
mpp-near agent clear-cache
```

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                     Agent Request                        │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
          ┌──────────────────────┐
          │  Check Session Cache │
          └──────────┬───────────┘
                     │
        ┌────────────┴────────────┐
        │ miss                    │ hit
        ▼                         ▼
┌──────────────────┐      ┌──────────────────┐
│ Initial Request  │      │ Use Cached Token │
└────────┬─────────┘      └──────────────────┘
         │
         │ 402
         ▼
┌──────────────────┐
│ Parse Challenge  │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│  Check Budget    │
└────────┬─────────┘
         │ ✓
         ▼
┌──────────────────┐
│ Pay via OutLayer │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│ Build Credential │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│  Retry Request   │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│  Cache Session   │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│   Return Data    │
└──────────────────┘
```

## Budget Controls

Prevent runaway costs with budget limits:

```rust
let budget = BudgetConfig {
    max_per_request: 0.10,    // Never pay more than $0.10 per request
    max_per_day: 5.00,        // Never spend more than $5.00 per day
    spent_today: 0.0,         // Tracked automatically
    require_approval_above: 0.50, // Require manual approval for >$0.50
    ..Default::default()
};

let client = AgentClient::new("wk_...")
    .with_budget(budget);
```

### Budget Methods

- `budget_status()` - Get current budget configuration
- `spent_today()` - Get amount spent today
- `remaining_budget()` - Get remaining daily budget
- `can_afford(amount)` - Check if amount is within budget

## Caching

### Session Caching

Avoid re-paying for the same resource:

```rust
// First request: pays and caches session token
let data1 = client.get("https://api.example.com/data").await?;

// Second request: uses cached token, no payment needed
let data2 = client.get("https://api.example.com/data").await?;
```

### Receipt Caching

Reuse payment proofs for identical challenges:

```rust
// First request: pays and caches receipt
let data1 = client.get("https://api.example.com/data").await?;

// If API returns same challenge: reuses cached receipt
let data2 = client.get("https://api.example.com/data").await?;
```

### Clear Cache

```rust
client.clear_cache();
```

Or via CLI:

```bash
mpp-near agent clear-cache
```

## Error Handling

```rust
use mpp_near::client::AgentError;

match client.get("https://paid-api.com/data").await {
    Ok(response) => {
        let data: serde_json::Value = response.json().await?;
        println!("{:?}", data);
    }
    Err(AgentError::BudgetExceeded { requested, available }) => {
        eprintln!("Budget exceeded: requested ${:.4}, available ${:.4}", 
            requested, available);
    }
    Err(AgentError::PaymentFailed(msg)) => {
        eprintln!("Payment failed: {}", msg);
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
}
```

## Configuration

### Environment Variables

```bash
export OUTLAYER_API_KEY="wk_..."
```

### Config File

```toml
# ~/.mpp-near/config.toml
[intents]
api_key = "wk_..."
api_url = "https://api.outlayer.fastnear.com"

[agent]
max_per_request = 0.10
max_per_day = 5.00
cache_enabled = true
```

## Advanced Usage

### Custom OutLayer URL

```rust
let client = AgentClient::new("wk_...")
    .with_outlayer_url("https://custom.outlayer.example.com");
```

### Disable Caching

```rust
let client = AgentClient::new("wk_...")
    .with_cache(false);
```

### Disable Auto-Pay

```rust
let client = AgentClient::new("wk_...")
    .with_auto_pay(false);

// Will return error on 402 instead of paying
match client.get("https://paid-api.com/data").await {
    Err(AgentError::PaymentFailed(msg)) => {
        // Handle manually
    }
    _ => {}
}
```

### Generic Request

```rust
use reqwest::Method;

let response = client.request(
    Method::PUT,
    "https://api.example.com/update",
    Some(&serde_json::json!({"field": "value"}))
).await?;
```

## CLI Reference

| Command | Description |
|---------|-------------|
| `agent get` | GET request with auto-payment |
| `agent post` | POST request with auto-payment |
| `agent budget` | Check/set budget limits |
| `agent test` | Test 402 flow (dry run) |
| `agent clear-cache` | Clear payment cache |

### Examples

```bash
# GET with custom budget
mpp-near agent get --url https://api.example.com/data --max 0.05

# POST with JSON body
mpp-near agent post \
    --url https://api.example.com/submit \
    --data '{"user":"alice","action":"process"}' \
    --max 0.10

# Set budget limits
mpp-near agent budget --set-max-request 0.50 --set-max-day 10.00

# Test endpoint (see challenge without paying)
mpp-near agent test --url https://api.example.com/data
```

## Best Practices

1. **Set budget limits** - Prevent runaway costs
2. **Use caching** - Avoid unnecessary payments
3. **Handle errors** - BudgetExceeded, PaymentFailed
4. **Test first** - Use `agent test` to see challenges
5. **Monitor spending** - Check `spent_today()` regularly

## Security

- API keys are never logged
- Payments use HMAC binding (replay protection)
- Sessions expire after 1 hour
- Budget limits prevent overspending
- OutLayer uses TEE (Trusted Execution Environment)

## Comparison: Manual vs AgentClient

| Task | Manual | AgentClient |
|------|--------|-------------|
| Detect 402 | Parse status code | Automatic |
| Parse challenge | Parse WWW-Authenticate | Automatic |
| Check budget | Manual check | Built-in |
| Pay challenge | Call OutLayer API | Automatic |
| Build credential | Base64 encode JSON | Automatic |
| Retry request | Add Authorization header | Automatic |
| Cache session | Manual storage | Automatic |

## Use Cases

### 1. AI Agent Accessing Paid APIs

```rust
// AI agent needs to access multiple paid APIs
let client = AgentClient::new("wk_...")
    .with_budget(BudgetConfig::new(0.10, 5.0));

// Fetch data from API 1
let api1_data = client.get("https://api1.example.com/data").await?;

// Fetch data from API 2
let api2_data = client.get("https://api2.example.com/data").await?;

// Submit results
let result = client.post("https://api3.example.com/submit", &json!({
    "api1": api1_data,
    "api2": api2_data
})).await?;
```

### 2. Automated Trading Bot

```rust
// Trading bot paying for market data
let client = AgentClient::new("wk_...")
    .with_budget(BudgetConfig::new(0.01, 1.0)); // $0.01 per request, $1.00 per day

// Get market data (auto-pays if required)
let prices = client.get("https://api.marketdata.com/prices").await?;

// Execute trade
let trade = client.post("https://api.exchange.com/trade", &json!({
    "symbol": "NEAR",
    "side": "buy",
    "amount": 10
})).await?;
```

### 3. Data Pipeline

```rust
// Data pipeline with multiple paid services
let client = AgentClient::new("wk_...")
    .with_budget(BudgetConfig::new(0.50, 10.0));

// Step 1: Fetch data
let raw_data = client.get("https://api.datasource.com/fetch").await?;

// Step 2: Process data (paid service)
let processed = client.post("https://api.processor.com/transform", &raw_data).await?;

// Step 3: Store results (paid service)
let stored = client.post("https://api.storage.com/save", &processed).await?;
```

## Troubleshooting

### "Budget exceeded"

Lower your request amount or increase daily limit:

```bash
mpp-near agent budget --set-max-day 20.00
```

### "Payment failed"

Check your OutLayer balance:

```bash
mpp-near balance
```

### "Challenge expired"

The challenge TTL (5 minutes) expired. Retry the request.

### "Invalid challenge"

The API returned a malformed 402 response. Contact the API provider.

## Resources

- **MPP-NEAR Repository**: https://github.com/mpp-near/mpp-near
- **OutLayer Dashboard**: https://outlayer.fastnear.com
- **Machine Payments Protocol**: https://mpp.dev
- **NEAR Blockchain**: https://near.org
