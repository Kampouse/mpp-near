---
name: mpp-near
description: NEAR payment CLI for Machine Payments Protocol (MPP). Send gasless payments via NEAR Intents, create payment checks for agent-to-agent payments, swap tokens across 20+ chains, and accept MPP payments. Use when an agent needs to handle NEAR blockchain payments - transfers, swaps, payment checks, or running a payment server.
metadata:
  api:
    base_url: https://api.outlayer.fastnear.com
    version: v1
    auth: Bearer token (wk_...)
  binaries:
    cli: mpp-near
    install: cargo install mpp-near --features client,intents
---

# MPP-NEAR CLI

NEAR payment CLI for Machine Payments Protocol. Enables gasless payments via NEAR Intents (OutLayer custody wallet), agent-to-agent payment checks, cross-chain token swaps, and HTTP 402 payment handling.

## When to Use This Skill

| You need... | Action |
|-------------|--------|
| A crypto wallet for your agent | Use `mpp-near register` to create custody wallet |
| Send NEAR/USDC/USDT payments | Use `mpp-near pay` --recipient, --amount, --token |
| Gasless payments (no gas tokens) | Use Intents provider (default) - solver pays gas |
| Agent-to-agent payments | Use `mpp-near create-check` + share check_key |
| Receive payment from another agent | Use `mpp-near claim-check` --check_key |
| Swap tokens (20+ chains) | Use `mpp-near swap` --from, --to, --amount |
| Accept MPP payments for APIs | Use `mpp-near server` to start payment server |
| Check wallet balance | Use `mpp-near balance` |
| Fund your wallet | Use `mpp-near fund-link` to generate funding URL |
| Configure wallet policies | Use `mpp-near handoff` to get management URL |

## Quick Start

### 1. Register Wallet (One-Time)

```bash
mpp-near register
```

**Response:**
```
✓ Wallet registered successfully!

  Wallet ID:     0a991095-aef4-476c-806a-5ea4a51650ab
  Account ID:    efa660437daaaae0e1fba740e2ebe5654613699ddb20bdc086e65025dca75129

  API Key:       wk_008272006f007bd9917b67cf429e11436e888c2575caae666ef2ce1586dfdf9d
ℹ IMPORTANT: Save your API key securely - it's shown only once!

Next steps:
  1. Save your API key: export MPP_NEAR_API_KEY=wk_...
  2. Fund your wallet: mpp-near fund-link --amount 0.1 --token near
  3. Check balance: mpp-near balance --api-key wk_...
```

### 2. Fund Wallet

```bash
# Fund with NEAR (for gas operations)
mpp-near fund-link --amount 0.1 --token near

# Fund with USDC to Intents balance (for gasless swaps)
mpp-near fund-link --amount 10 --token usdc --intents
```

**Auto-opens browser** to complete funding transaction.

### 3. Check Balance

```bash
mpp-near balance --api-key wk_...
```

**Response:**
```
✓ Balance retrieved
  Account: 5c571cf253c3edb672df980cc56078f2c455b972cc01ac34af51e95967ba6edb
  Balance: 0.100000 NEAR
  USDC:    10.000000
```

### 4. Send Payment (Gasless!)

```bash
mpp-near pay --recipient merchant.near --amount 0.001 --token near
mpp-near pay --recipient merchant.near --amount 1 --token usdc --memo "Payment for services"
```

## Configuration

### API Key Authentication

```bash
# Set API key (recommended)
export MPP_NEAR_API_KEY=wk_...

# Or pass with each command
mpp-near balance --api-key wk_...
```

### Config File

Create `~/.mpp-near/config.toml`:

```toml
method = "intents"

[intents]
api_key = "wk_..."
```

## Commands

### Register Wallet

```bash
mpp-near register
```

**Creates** a new OutLayer custody wallet.

### Generate Funding Link

```bash
mpp-near fund-link --amount <AMOUNT> --token <TOKEN> [--intents] [--memo <TEXT>]
```

**Generates** a browser-based funding URL.

**Examples:**
```bash
# Fund NEAR for gas
mpp-near fund-link --amount 0.1 --token near

# Fund USDC to Intents (for swaps/checks)
mpp-near fund-link --amount 10 --token usdc --intents --memo "Gasless operations"
```

### Check Balance

```bash
mpp-near balance [--api-key wk_...]
```

**Shows** NEAR and USDC balances.

### Send Payment

```bash
mpp-near pay --recipient <ACCOUNT> --amount <AMOUNT> --token <TOKEN> [--memo <TEXT>]
```

**Sends** gasless payment via NEAR Intents.

**Examples:**
```bash
# Send NEAR
mpp-near pay --recipient merchant.near --amount 0.001 --memo "Payment for API call"

# Send USDC
mpp-near pay --recipient merchant.near --amount 5 --token usdc --memo "Monthly subscription"
```

### Create Payment Check

```bash
mpp-near create-check --amount <AMOUNT> --token <TOKEN> [--memo <TEXT>] [--expires-in <SECONDS>]
```

**Creates** a claimable payment check (agent-to-agent payment).

**Examples:**
```bash
# Create 1 USDC check
mpp-near create-check --amount 1 --token usdc --memo "AI service payment"

# Create check with 1-hour expiry
mpp-near create-check --amount 0.5 --token usdc --expires-in 3600
```

**Response:**
```
✓ Payment check created

  Check ID:  be291056-12c0-41f4-b4cc-5316c66a5dd1
  Check Key: b72e42e49bc15cdde9b195c0849d9d776cbbde0c077dd0fce10791da518f29c2
  Amount:    1 USDC
  Memo:      AI service payment
  Expires:   2026-03-20T19:59:16

Share the check key with the recipient to claim.
```

### Claim Payment Check

```bash
mpp-near claim-check --check-key <KEY> [--amount <AMOUNT>]
```

**Claims** a payment check received from another agent.

**Examples:**
```bash
# Claim full check
mpp-near claim-check --check-key b72e42e49bc15cdde9b195c0849d9d776cbbde0c077dd0fce10791da518f29c2

# Claim partial amount
mpp-near claim-check --check-key b72e42e...c2 --amount 0.5
```

### Swap Tokens (Gasless)

```bash
mpp-near swap --from <TOKEN> --to <TOKEN> --amount <AMOUNT>
```

**Swaps** tokens across 20+ chains via NEAR Intents (gasless).

**Examples:**
```bash
# Swap USDC to NEAR
mpp-near swap --from usdc --to near --amount 1

# Swap NEAR to USDC
mpp-near swap --from near --to usdc --amount 0.1

# Swap USDT to wNEAR
mpp-near swap --from usdt --to near --amount 10
```

**Response:**
```
✓ Swap completed (gasless)!
  Request ID: ec9ff45d-55b8-4ebb-ae03-fca23fc11602
  Amount out: 0.372251 NEAR
  Intent:     HBFd9rds4sGECRabQdoLF7sShEdEBuMLpmRGq23znbfA
```

### List Available Tokens

```bash
mpp-near tokens
```

**Lists** 151+ tokens available for swaps.

### Show Wallet Management URL

```bash
mpp-near handoff
```

**Shows** the OutLayer dashboard URL.

## Gas Model

| Operation | Gas Required | Who Pays |
|-----------|--------------|----------|
| `pay` (intents) | **No** | Solver relay |
| `create-check` | **No** | Solver relay |
| `claim-check` | **No** | Solver relay |
| `swap` | **No** | Solver relay |
| `balance` | No | N/A (read-only) |

**All operations are gasless by default!** 🎉

## Best Practices

1. **Use payment checks** for agent-to-agent payments (no storage needed)
2. **Set expiry on checks** - 86400s (24h) default
3. **Fund with --intents** for gasless operations
4. **Never share API keys** - Treat like private keys

## Examples

### Complete Payment Flow

```bash
# 1. Register wallet
mpp-near register

# 2. Fund wallet
mpp-near fund-link --amount 0.1 --token near
mpp-near fund-link --amount 10 --token usdc --intents

# 3. Check balance
mpp-near balance

# 4. Send payment (gasless!)
mpp-near pay --recipient merchant.near --amount 0.001

# 5. Create payment check
mpp-near create-check --amount 1 --token usdc --memo "Service payment"

# 6. Swap tokens
mpp-near swap --from usdc --to near --amount 0.5
```

### Agent-to-Agent Payment

```bash
# Agent A (sender)
mpp-near create-check --amount 5 --token usdc --memo "Data processing"
# → Share check_key with Agent B

# Agent B (recipient)
mpp-near claim-check --check-key <KEY_FROM_AGENT_A>
```

## Troubleshooting

### Storage Registration Required

**Solution:** Use payment checks instead (no storage needed):
```bash
mpp-near create-check --amount 1 --token usdc --memo "Payment"
```

### Insufficient Balance

**Solution:** Generate funding link:
```bash
mpp-near fund-link --amount 1 --token near
```

## References

- **Repository:** https://github.com/kampouse/mpp-near
- **OutLayer Dashboard:** https://outlayer.fastnear.com
- **MPP Protocol:** https://mpp.dev

## Version

Current version: 0.1.0

## License

MIT OR Apache-2.0
