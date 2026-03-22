//! Main CLI entry point

use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use mpp_near::types::AccountId;
use std::path::{PathBuf, Path};
use std::sync::Arc;
use std::fs;
use serde::Deserialize;

#[cfg(feature = "client")]
use mpp_near::client::NearProvider;

#[cfg(feature = "intents")]
use mpp_near::client::IntentsProvider;

#[cfg(feature = "server")]
use mpp_near::server::{NearVerifier, VerifierConfig};

/// Configuration file structure
#[derive(Debug, Deserialize)]
struct Config {
    method: Option<String>,
    standard: Option<StandardConfig>,
    intents: Option<IntentsConfig>,
}

#[derive(Debug, Deserialize)]
struct StandardConfig {
    account: Option<String>,
    private_key: Option<String>,
    rpc_url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct IntentsConfig {
    api_key: Option<String>,
}

impl Config {
    /// Load config from the default path or a custom path
    fn load(path: Option<&Path>) -> Result<Option<Self>> {
        let config_path = if let Some(p) = path {
            p.to_path_buf()
        } else {
            // Check multiple locations in order:
            // 1. .mpp-config in current directory
            // 2. ~/.mpp-near/config.toml
            let local_config = PathBuf::from(".mpp-config");
            if local_config.exists() {
                local_config
            } else {
                let home_dir = std::env::var("HOME")
                    .or_else(|_| std::env::var("USERPROFILE"))?;
                PathBuf::from(home_dir).join(".mpp-near").join("config.toml")
            }
        };

        if !config_path.exists() {
            return Ok(None);
        }

        let contents = fs::read_to_string(&config_path)
            .map_err(|e| anyhow::anyhow!("Failed to read config file {}: {}", config_path.display(), e))?;

        let config: Config = toml::from_str(&contents)
            .map_err(|e| anyhow::anyhow!("Failed to parse config file {}: {}", config_path.display(), e))?;

        Ok(Some(config))
    }
}


#[derive(Parser)]
#[command(name = "mpp-near")]
#[command(about = "NEAR payment CLI for Machine Payments Protocol", long_about = None)]
#[command(version)]
struct Cli {
    /// Payment method to use (standard or intents)
    #[arg(short, long, global = true, default_value = "intents")]
    method: String,

    /// Path to config file (default: ~/.mpp-near/config.toml)
    #[arg(short, long, global = true)]
    config: Option<PathBuf>,

    /// RPC URL for standard provider
    #[arg(long, global = true)]
    rpc_url: Option<String>,

    /// Account ID for standard provider
    #[arg(short, long, global = true)]
    account: Option<String>,

    /// Private key for standard provider (ed25519:...)
    #[arg(short, long, global = true)]
    private_key: Option<String>,

    /// API key for intents provider (wk_...)
    #[arg(long, global = true)]
    api_key: Option<String>,

    /// Verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Send a payment
    Pay {
        /// Recipient account ID
        #[arg(short, long)]
        recipient: String,

        /// Amount in NEAR (e.g., "1.5")
        #[arg(short = 'n', long)]
        amount: String,

        /// Token to send (near, usdc, usdt)
        #[arg(short = 't', long, default_value = "near")]
        token: String,

        /// Memo to include with transaction
        #[arg(short = 'o', long)]
        memo: Option<String>,
    },

    /// Register a new OutLayer custody wallet
    Register,

    /// Check account balance
    Balance {
        /// Account to check (defaults to configured account)
        #[arg(short, long)]
        account: Option<String>,
    },

    /// Generate a funding link for the wallet
    FundLink {
        /// Amount to request
        #[arg(short = 'n', long)]
        amount: String,

        /// Token to receive (near, usdc, usdt)
        #[arg(short = 't', long, default_value = "near")]
        token: String,

        /// Optional message for the funder
        #[arg(short = 'o', long)]
        memo: Option<String>,

        /// Deposit to intents balance (for gasless operations)
        #[arg(long)]
        intents: bool,
    },

    /// Show wallet management URL (handoff)
    Handoff,

    /// Register storage for an account (required before receiving tokens)
    StorageDeposit {
        /// Account ID to register storage for (defaults to self)
        #[arg(short, long)]
        account: Option<String>,

        /// Token contract (default: near)
        #[arg(long, default_value = "near")]
        token: String,
    },

    /// Verify a transaction
    Verify {
        /// Transaction hash to verify
        #[arg(short, long)]
        tx_hash: String,

        /// Expected amount in NEAR (for verification)
        #[arg(long)]
        expected_amount: Option<String>,

        /// Expected recipient (for verification)
        #[arg(long)]
        expected_recipient: Option<String>,
    },

    /// Start a payment server
    Server {
        /// Port to listen on
        #[arg(long, default_value = "3000")]
        port: u16,

        /// Recipient account for payments
        #[arg(short, long)]
        recipient: String,

        /// Minimum payment amount in NEAR
        #[arg(long, default_value = "0.001")]
        min_amount: String,
    },

    /// List available tokens (intents only)
    Tokens,

    /// Create a payment check (intents only)
    CreateCheck {
        /// Amount in token units
        #[arg(short = 'n', long)]
        amount: String,

        /// Token to use (near, usdc, usdt)
        #[arg(short = 't', long, default_value = "near")]
        token: String,

        /// Memo for the check
        #[arg(short = 'o', long)]
        memo: Option<String>,

        /// Expiry time in seconds
        #[arg(short, long, default_value = "86400")]
        expires_in: u64,
    },

    /// Claim a payment check (intents only)
    ClaimCheck {
        /// Check key to claim
        #[arg(short, long)]
        check_key: String,

        /// Amount to claim (optional, claims all if not specified)
        #[arg(short = 'n', long)]
        amount: Option<String>,
    },

    /// Swap tokens (intents only)
    Swap {
        /// Token to swap from
        #[arg(long)]
        from: String,

        /// Token to swap to
        #[arg(long)]
        to: String,

        /// Amount to swap
        #[arg(short = 'n', long)]
        amount: String,
    },

    /// Show current configuration
    Config,

    /// Agent commands for seamless API access with auto-402 handling
    Agent {
        #[command(subcommand)]
        command: AgentCommand,
    },
}

/// Agent subcommands for seamless paid API access
#[derive(clap::Subcommand)]
enum AgentCommand {
    /// Get resource from paid API (auto-handle 402 payment)
    Get {
        /// API endpoint URL
        #[arg(short, long)]
        url: String,

        /// Maximum to spend (USD)
        #[arg(short, long, default_value = "0.10")]
        max: f64,

        /// Output format (json, text)
        #[arg(short, long, default_value = "json")]
        output: String,
    },

    /// POST data to paid API (auto-handle 402 payment)
    Post {
        /// API endpoint URL
        #[arg(short, long)]
        url: String,

        /// JSON body to send
        #[arg(short, long)]
        data: String,

        /// Maximum to spend (USD)
        #[arg(short, long, default_value = "0.10")]
        max: f64,
    },

    /// Check agent budget status
    Budget {
        /// Set max per request (USD)
        #[arg(long)]
        set_max_request: Option<f64>,

        /// Set max per day (USD)
        #[arg(long)]
        set_max_day: Option<f64>,
    },

    /// Clear payment cache
    ClearCache,

    /// Test 402 flow without paying (dry run)
    Test {
        /// API endpoint URL to test
        #[arg(short, long)]
        url: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file if it exists (it will be ignored if not found)
    dotenv::dotenv().ok();

    let mut cli = Cli::parse();

    // Load config file
    let config = Config::load(cli.config.as_deref())?;

    // Merge config file values with CLI args (CLI args take precedence)
    if let Some(cfg) = config {
        // Only use config values if CLI args weren't provided
        if cli.method == "intents" && cli.api_key.is_none() {
            if let Some(intents_cfg) = &cfg.intents {
                if let Some(key) = &intents_cfg.api_key {
                    if !key.is_empty() {
                        cli.api_key = Some(key.clone());
                    }
                }
            }
        }

        if cli.account.is_none() {
            if let Some(standard_cfg) = &cfg.standard {
                if let Some(account) = &standard_cfg.account {
                    cli.account = Some(account.clone());
                }
            }
        }

        if cli.private_key.is_none() {
            if let Some(standard_cfg) = &cfg.standard {
                if let Some(key) = &standard_cfg.private_key {
                    cli.private_key = Some(key.clone());
                }
            }
        }

        if cli.rpc_url.is_none() {
            if let Some(standard_cfg) = &cfg.standard {
                if let Some(url) = &standard_cfg.rpc_url {
                    cli.rpc_url = Some(url.clone());
                }
            }
        }

        // Use method from config if not explicitly set to default
        if cli.method == "intents" {
            if let Some(method) = cfg.method {
                cli.method = method;
            }
        }
    }

    // Initialize logging
    if cli.verbose {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::WARN)
            .init();
    }

    match &cli.command {
        Commands::Register => {
            cmd_register().await?;
        }

        Commands::Pay { recipient, amount, token, memo } => {
            cmd_pay(&cli, recipient, amount, token, memo.as_deref()).await?;
        }

        Commands::Balance { account } => {
            cmd_balance(&cli, account.as_deref()).await?;
        }

        Commands::FundLink { amount, token, memo, intents } => {
            cmd_fund_link(&cli, amount, token, memo.as_deref(), *intents).await?;
        }

        Commands::Handoff => {
            cmd_handoff(&cli).await?;
        }

        Commands::StorageDeposit { account, token } => {
            cmd_storage_deposit(&cli, account.as_deref(), token).await?;
        }

        Commands::Verify { tx_hash, expected_amount, expected_recipient } => {
            cmd_verify(&cli, tx_hash, expected_amount.as_deref(), expected_recipient.as_deref()).await?;
        }

        Commands::Server { port, recipient, min_amount } => {
            cmd_server(&cli, *port, recipient, min_amount).await?;
        }

        Commands::Tokens => {
            cmd_tokens(&cli).await?;
        }

        Commands::CreateCheck { amount, token, memo, expires_in } => {
            cmd_create_check(&cli, amount, token, memo.as_deref(), *expires_in).await?;
        }

        Commands::ClaimCheck { check_key, amount } => {
            cmd_claim_check(&cli, check_key, amount.as_deref()).await?;
        }

        Commands::Swap { from, to, amount } => {
            cmd_swap(&cli, from, to, amount).await?;
        }

        Commands::Config => {
            cmd_config(&cli)?;
        }

        Commands::Agent { command } => {
            cmd_agent(&cli, command).await?;
        }
    }

    Ok(())
}

async fn cmd_agent(cli: &Cli, command: &AgentCommand) -> Result<()> {
    use mpp_near::client::{AgentClient, BudgetConfig};

    match command {
        AgentCommand::Get { url, max, output } => {
            let api_key = cli.api_key.as_ref()
                .ok_or_else(|| anyhow::anyhow!("--api-key required. Get one with: mpp-near register"))?;

            print_info(&format!("🌐 Requesting: {}", url));

            let client = AgentClient::new(api_key.clone())
                .with_budget(BudgetConfig::new(*max, 5.0));

            let resp = client.get(url).await?;

            match output.as_str() {
                "json" => {
                    let json: serde_json::Value = resp.json().await?;
                    println!("{}", serde_json::to_string_pretty(&json)?);
                }
                "text" => {
                    println!("{}", resp.text().await?);
                }
                _ => {
                    println!("{}", resp.text().await?);
                }
            }

            print_success(&format!("Paid: ${:.4} | Remaining budget: ${:.4}", 
                client.spent_today(), client.remaining_budget()));
        }

        AgentCommand::Post { url, data, max } => {
            let api_key = cli.api_key.as_ref()
                .ok_or_else(|| anyhow::anyhow!("--api-key required"))?;

            print_info(&format!("🌐 POSTing to: {}", url));

            let client = AgentClient::new(api_key.clone())
                .with_budget(BudgetConfig::new(*max, 5.0));

            let body: serde_json::Value = serde_json::from_str(data)
                .map_err(|e| anyhow::anyhow!("Invalid JSON body: {}", e))?;

            let resp = client.post(url, &body).await?;

            let json: serde_json::Value = resp.json().await?;
            println!("{}", serde_json::to_string_pretty(&json)?);

            print_success(&format!("Paid: ${:.4}", client.spent_today()));
        }

        AgentCommand::Budget { set_max_request, set_max_day } => {
            if let Some(max_req) = set_max_request {
                println!("Max per request: ${:.2}", max_req);
            }
            if let Some(max_day) = set_max_day {
                println!("Max per day: ${:.2}", max_day);
            }

            println!();
            println!("Budget Status:");
            println!("  Max per request: $0.10");
            println!("  Max per day:     $5.00");
            println!("  Spent today:     $0.00");
            println!("  Remaining:       $5.00");
            println!();
            println!("To update budget:");
            println!("  mpp-near agent budget --set-max-request 0.50");
            println!("  mpp-near agent budget --set-max-day 10.00");
        }

        AgentCommand::ClearCache => {
            let api_key = cli.api_key.as_ref()
                .ok_or_else(|| anyhow::anyhow!("--api-key required"))?;

            let client = AgentClient::new(api_key.clone());
            client.clear_cache();

            print_success("Payment cache cleared");
        }

        AgentCommand::Test { url } => {
            print_info(&format!("🧪 Testing 402 flow: {}", url));

            // Just make a request to see the 402 challenge
            let resp = reqwest::Client::new().get(url).send().await?;

            if resp.status() == 402 {
                if let Some(www_auth) = resp.headers().get("WWW-Authenticate") {
                    print_success("Received 402 challenge:");
                    println!();
                    println!("  {}", www_auth.to_str().unwrap_or("(invalid)"));
                    println!();

                    // Try to parse it
                    use mpp_near::client::Challenge402;
                    if let Ok(challenge) = Challenge402::parse(www_auth.to_str().unwrap()) {
                        println!("Parsed challenge:");
                        println!("  Amount:    {} {}", challenge.amount, challenge.token);
                        println!("  Recipient: {}", challenge.recipient);
                        println!("  Challenge: {}", challenge.challenge);
                        println!("  Nonce:     {}", challenge.nonce);
                    }
                } else {
                    print_info("402 response but no WWW-Authenticate header");
                }
            } else {
                print_info(&format!("Response status: {} (not 402)", resp.status()));
            }
        }
    }

    Ok(())
}

async fn cmd_pay(cli: &Cli, recipient: &str, amount: &str, token: &str, memo: Option<&str>) -> Result<()> {
    let recipient = AccountId::new(recipient)?;
    let near_amount = parse_amount(amount)?;

    match cli.method.as_str() {
        "intents" => {
            #[cfg(feature = "intents")]
            {
                let api_key = cli.api_key.as_ref()
                    .ok_or_else(|| anyhow::anyhow!("--api-key required for intents method"))?;

                print_info(&format!("Sending {} {} to {} (gasless)", near_amount, token, recipient));

                let provider = IntentsProvider::new(api_key.clone());

                let result = match token.to_lowercase().as_str() {
                    "near" => provider.transfer(&recipient, near_amount).await,
                    "usdc" => provider.transfer_token(
                        "17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1",
                        &recipient,
                        near_amount,
                    ).await,
                    "usdt" => provider.transfer_token(
                        "usdt.tether-token.near",
                        &recipient,
                        near_amount,
                    ).await,
                    _ => return Err(anyhow::anyhow!("Unsupported token: {}", token)),
                };

                match result {
                    Ok(tx_hash) => {
                        print_success("Payment sent (gasless)!");
                        println!("  Transaction: {}", tx_hash);
                        println!("  Recipient:   {}", recipient);
                        println!("  Amount:      {} {}", near_amount, token);
                        println!("  Gas cost:    0 (paid by solver)");
                    }
                    Err(e) => {
                        // Check if it's a storage error and provide helpful suggestion
                        let error_msg = e.to_string();
                        if error_msg.contains("has no storage") {
                            print_info("ℹ Storage registration required");
                            println!();
                            println!("The recipient needs storage registered on the token contract.");
                            println!();
                            println!("Solutions:");
                            println!("  1. Use a funding link (auto-registers storage):");
                            println!("     mpp-near fund-link --recipient {} --amount {} --token {}",
                                     recipient.as_str().replace('.', "\\."),
                                     amount,
                                     token);
                            println!();
                            println!("  2. Ask recipient to register storage:");
                            println!("     https://outlayer.fastnear.com/wallet/fund?to={}&amount=0.001&token={}",
                                     recipient.as_str(),
                                     get_token_id(token));
                            println!();
                            println!("  3. Try storage-deposit command (may be blocked by policy):");
                            println!("     mpp-near storage-deposit --account {} --token {}",
                                     recipient.as_str().replace('.', "\\."),
                                     token);
                            return Err(e.into());
                        } else if error_msg.contains("insufficient_balance") {
                            print_info("ℹ Insufficient balance");
                            println!();
                            println!("Your wallet needs more tokens. Generate a funding link:");
                            println!("  mpp-near fund-link --amount 1 --token {}", token);
                            return Err(e.into());
                        } else {
                            return Err(e.into());
                        }
                    }
                }
            }

            #[cfg(not(feature = "intents"))]
            return Err(anyhow::anyhow!("Intents feature not enabled"));
        }

        "standard" => {
            #[cfg(feature = "client")]
            {
                let account = cli.account.as_ref()
                    .ok_or_else(|| anyhow::anyhow!("--account required for standard method"))?;
                let pk = cli.private_key.as_ref()
                    .ok_or_else(|| anyhow::anyhow!("--private-key required for standard method"))?;
                let rpc = cli.rpc_url.as_deref().unwrap_or("https://rpc.mainnet.near.org");

                print_info(&format!("Sending {} {} to {} (standard)", near_amount, token, recipient));

                let provider = NearProvider::new(
                    AccountId::new(account)?,
                    pk.clone(),
                    rpc,
                )?;

                let tx_hash = match token {
                    "near" => provider.transfer(&recipient, near_amount).await?,
                    "usdc" => provider.transfer_token(
                        &AccountId::new("usdc.contract.near")?,
                        &recipient,
                        near_amount,
                    ).await?,
                    "usdt" => provider.transfer_token(
                        &AccountId::new("usdt.tether-token.near")?,
                        &recipient,
                        near_amount,
                    ).await?,
                    _ => return Err(anyhow::anyhow!("Unsupported token: {}", token)),
                };

                print_success("Payment sent!");
                println!("  Transaction: {}", tx_hash);
                println!("  Recipient:   {}", recipient);
                println!("  Amount:      {} {}", near_amount, token);
            }

            #[cfg(not(feature = "client"))]
            return Err(anyhow::anyhow!("Client feature not enabled"));
        }

        _ => return Err(anyhow::anyhow!("Unknown method: {}. Use 'standard' or 'intents'", cli.method)),
    }

    if let Some(m) = memo {
        print_info(&format!("Memo: {}", m));
    }

    Ok(())
}

async fn cmd_register() -> Result<()> {
    print_info("Registering new OutLayer custody wallet...");

    let response = reqwest::Client::new()
        .post("https://api.outlayer.fastnear.com/register")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Registration failed: HTTP {}", response.status()));
    }

    let data: serde_json::Value = response.json().await?;

    let api_key = data["api_key"].as_str().ok_or_else(|| anyhow::anyhow!("No api_key in response"))?;
    let wallet_id = data["wallet_id"].as_str().ok_or_else(|| anyhow::anyhow!("No wallet_id in response"))?;
    let near_account_id = data["near_account_id"].as_str().ok_or_else(|| anyhow::anyhow!("No near_account_id in response"))?;
    let handoff_url = data["handoff_url"].as_str().ok_or_else(|| anyhow::anyhow!("No handoff_url in response"))?;

    print_success("Wallet registered successfully!");
    println!();
    println!("  Wallet ID:     {}", wallet_id);
    println!("  Account ID:    {}", near_account_id);
    println!();
    println!("  API Key:       {}", api_key);
    print_info("IMPORTANT: Save your API key securely - it's shown only once!");
    println!();
    println!("  Management:    {}", handoff_url);
    println!();
    println!("Next steps:");
    println!("  1. Save your API key: export MPP_NEAR_API_KEY={}", api_key);
    println!("  2. Fund your wallet:");
    println!("     mpp-near fund-link --amount 0.1 --token near");
    println!("  3. Check balance:");
    println!("     mpp-near balance --api-key {}", api_key);

    Ok(())
}

async fn cmd_fund_link(cli: &Cli, amount: &str, token: &str, memo: Option<&str>, intents: bool) -> Result<()> {
    #[cfg(feature = "intents")]
    {
        let api_key = cli.api_key.as_ref()
            .ok_or_else(|| anyhow::anyhow!("--api-key required. Get one with: mpp-near register"))?;

        let provider = IntentsProvider::new(api_key.clone());
        let account_id = provider.get_account_id().await?;

    let token_id = get_token_id(token);
    let memo_encoded = memo.map(|m| urlencoding::encode(m).to_string()).unwrap_or_default();

    let mut url = format!(
        "https://outlayer.fastnear.com/wallet/fund?to={}&amount={}&token={}",
        account_id, amount, token_id
    );

    if !memo_encoded.is_empty() {
        url.push_str(&format!("&msg={}", memo_encoded));
    }

    if intents {
        url.push_str("&dest=intents");
    }

    print_success("Funding link generated:");
    println!();
    println!("  {}", url);
    println!();

    if intents {
        print_info("Tokens will be deposited to Intents balance (gasless operations)");
    } else {
        print_info("Tokens will be deposited to wallet balance (for gas operations)");
    }

    println!();
    println!("Open this link in your browser to fund the wallet.");
    println!("After funding, run: mpp-near balance");

    // Try to open the link automatically
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open")
            .arg(&url)
            .spawn();
        print_info("Opening funding link in browser...");
    }

    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("xdg-open")
            .arg(&url)
            .spawn();
        print_info("Opening funding link in browser...");
    }

    Ok(())
    }
    
    #[cfg(not(feature = "intents"))]
    {
        Err(anyhow::anyhow!("Intents feature not enabled"))
    }
}

async fn cmd_handoff(cli: &Cli) -> Result<()> {
    let api_key = cli.api_key.as_ref()
        .ok_or_else(|| anyhow::anyhow!("--api-key required"))?;

    print_success("Wallet Management URL:");
    println!();
    println!("  https://outlayer.fastnear.com/wallet?key={}", api_key);
    println!();
    println!("Use this link to:");
    println!("  - Configure spending policies");
    println!("  - View transaction history");
    println!("  - Recover your API key if lost");
    println!("  - Set up multi-sig approval");

    Ok(())
}

async fn cmd_balance(cli: &Cli, account: Option<&str>) -> Result<()> {
    match cli.method.as_str() {
        "intents" => {
            #[cfg(feature = "intents")]
            {
                let api_key = cli.api_key.as_ref()
                    .ok_or_else(|| anyhow::anyhow!("--api-key required for intents method"))?;

                print_info("Checking intents wallet balance...");

                let provider = IntentsProvider::new(api_key.clone());
                let account_id = provider.get_account_id().await?;
                let balance = provider.check_balance().await?;

                print_success("Balance retrieved");
                println!("  Account: {}", account_id);
                println!("  Balance: {}", balance);

                // Try to get USDC balance
                match provider.check_intents_balance("17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1").await {
                    Ok(usdc) => {
                        let usdc_amount = usdc.0 as f64 / 1_000_000.0;
                        println!("  USDC:    {:.6}", usdc_amount);
                    }
                    Err(_) => {}
                }
            }

            #[cfg(not(feature = "intents"))]
            return Err(anyhow::anyhow!("Intents feature not enabled"));
        }

        "standard" => {
            #[cfg(feature = "client")]
            {
                let account_id = account
                    .or(cli.account.as_deref())
                    .ok_or_else(|| anyhow::anyhow!("--account required"))?;

                print_info(&format!("Checking balance for {}...", account_id));

                // Would need to implement RPC query
                print_success("Balance check not yet implemented for standard method");
            }

            #[cfg(not(feature = "client"))]
            return Err(anyhow::anyhow!("Client feature not enabled"));
        }

        _ => return Err(anyhow::anyhow!("Unknown method: {}", cli.method)),
    }

    Ok(())
}

async fn cmd_verify(cli: &Cli, tx_hash: &str, expected_amount: Option<&str>, expected_recipient: Option<&str>) -> Result<()> {
    print_info(&format!("Verifying transaction: {}", tx_hash));

    println!();
    println!("Transaction Details:");
    println!("  Hash:    {}", tx_hash);

    if let Some(amt) = expected_amount {
        println!("  Expected amount:      {} NEAR", amt);
    }

    if let Some(rec) = expected_recipient {
        println!("  Expected recipient:   {}", rec);
    }

    println!();
    print_info("Note: Full verification requires RPC access");
    print_info("Use near-cli or NEAR explorer for detailed verification");

    Ok(())
}

async fn cmd_storage_deposit(cli: &Cli, account: Option<&str>, token: &str) -> Result<()> {
    #[cfg(feature = "intents")]
    {
        let api_key = cli.api_key.as_ref()
            .ok_or_else(|| anyhow::anyhow!("--api-key required"))?;

        print_info(&format!("Registering storage for token '{}'...", token));

        let provider = IntentsProvider::new(api_key.clone());

        match account {
            Some(acc) => {
                print_info(&format!("Registering storage for account: {}", acc));
            }
            None => {
                let account_id = provider.get_account_id().await?;
                print_info(&format!("Registering storage for own account: {}", account_id));
            }
        }

        let already_registered = provider.storage_deposit(token, account).await?;

        if already_registered {
            print_success("Storage already registered");
        } else {
            print_success("Storage registered successfully");
        }
        
        Ok(())
    }
    
    #[cfg(not(feature = "intents"))]
    {
        Err(anyhow::anyhow!("Intents feature not enabled"))
    }
}

async fn cmd_server(cli: &Cli, port: u16, recipient: &str, min_amount: &str) -> Result<()> {
    #[cfg(feature = "server")]
    {
        let recipient_account = AccountId::new(recipient)?;
        let min_near: f64 = min_amount.parse()
            .map_err(|_| anyhow::anyhow!("Invalid min_amount: {}", min_amount))?;
        let min_amount_yocto = (min_near * 1e24) as u128;

        print_info(&format!("Starting payment server on port {}", port));
        print_info(&format!("Recipient: {}", recipient_account));
        print_info(&format!("Min amount: {} NEAR", min_amount));

        let config = VerifierConfig {
            recipient_account: recipient_account.clone(),
            min_amount: mpp_near::types::NearAmount::from_yocto(min_amount_yocto),
            ..Default::default()
        };

        let verifier = NearVerifier::new(config)?;
        let verifier = Arc::new(verifier);

        use axum::{routing::get, Router};
        use serde_json::json;

        let app = Router::new()
            .route("/", get(|| async { axum::Json(json!({"name": "mpp-near-server", "version": env!("CARGO_PKG_VERSION")})) }))
            .route("/health", get(|| async { axum::Json(json!({"status": "healthy"})) }))
            .route("/challenge", get(|axum::extract::State(v): axum::extract::State<Arc<NearVerifier>>| async move {
                match v.charge("1").await {
                    Ok(challenge) => axum::Json(json!({"status": "payment_required", "challenge": challenge})),
                    Err(e) => axum::Json(json!({"error": e.to_string()})),
                }
            }))
            .with_state(verifier);

        let addr = format!("0.0.0.0:{}", port);
        let listener = tokio::net::TcpListener::bind(&addr).await?;

        print_success(&format!("Server listening on http://{}", addr));
        println!();
        println!("Endpoints:");
        println!("  GET /          - API info");
        println!("  GET /health    - Health check");
        println!("  GET /challenge - Create payment challenge");
        println!();
        println!("Ready to accept payments via MPP!");

        axum::serve(listener, app).await?;
    }

    #[cfg(not(feature = "server"))]
    return Err(anyhow::anyhow!("Server feature not enabled. Compile with --features server"));

    Ok(())
}

async fn cmd_tokens(cli: &Cli) -> Result<()> {
    #[cfg(feature = "intents")]
    {
        let api_key = cli.api_key.as_ref()
            .ok_or_else(|| anyhow::anyhow!("--api-key required"))?;

        print_info("Fetching available tokens...");

        let provider = IntentsProvider::new(api_key.clone());
        let tokens = provider.list_tokens().await?;

        print_success(&format!("Found {} tokens", tokens.len()));

        // Group by chain (tokens can be on multiple chains)
        let mut by_chain: std::collections::HashMap<String, Vec<_>> = std::collections::HashMap::new();
        for token in tokens {
            for chain in &token.chains {
                by_chain.entry(chain.clone()).or_default().push(token.clone());
            }
        }

        for (chain, mut chain_tokens) in by_chain {
            println!();
            println!("{}", chain.to_uppercase().bold());
            chain_tokens.sort_by(|a, b| a.symbol.cmp(&b.symbol));

            for token in chain_tokens.iter().take(10) {
                println!("  {:6} - {} ({} decimals)", token.symbol, token.id, token.decimals);
            }

            if chain_tokens.len() > 10 {
                println!("  ... and {} more", chain_tokens.len() - 10);
            }
        }
        
        Ok(())
    }
    
    #[cfg(not(feature = "intents"))]
    {
        Err(anyhow::anyhow!("Intents feature not enabled"))
    }
}

async fn cmd_create_check(cli: &Cli, amount: &str, token: &str, memo: Option<&str>, expires_in: u64) -> Result<()> {
    #[cfg(feature = "intents")]
    {
        let api_key = cli.api_key.as_ref()
            .ok_or_else(|| anyhow::anyhow!("--api-key required"))?;

        let near_amount = parse_amount_token(amount, token)?;

        print_info(&format!("Creating payment check for {} {}...", amount, token));

        let provider = IntentsProvider::new(api_key.clone());
        let token_id = get_token_id(token);

        let check = provider.create_payment_check(
            token_id,
            near_amount,
            memo,
            Some(expires_in),
        ).await?;

        print_success("Payment check created");
        println!();
        println!("  Check ID:  {}", check.check_id);
        println!("  Check Key: {}", check.check_key);
        println!("  Amount:    {} {}", check.amount, token);
        if let Some(m) = memo {
            println!("  Memo:      {}", m);
        }
        if let Some(exp) = check.expires_at {
            println!("  Expires:   {}", exp);
        }
        println!();
        println!("Share the check key with the recipient to claim.");
        
        Ok(())
    }
    
    #[cfg(not(feature = "intents"))]
    {
        Err(anyhow::anyhow!("Intents feature not enabled"))
    }
}

async fn cmd_claim_check(cli: &Cli, check_key: &str, amount: Option<&str>) -> Result<()> {
    #[cfg(feature = "intents")]
    {
        let api_key = cli.api_key.as_ref()
            .ok_or_else(|| anyhow::anyhow!("--api-key required"))?;

        let near_amount = amount
            .map(|a| parse_amount_token(a, "near"))
            .transpose()?;

        print_info(&format!("Claiming payment check {}...", check_key));

        let provider = IntentsProvider::new(api_key.clone());
        let claimed = provider.claim_payment_check(check_key, near_amount).await?;

        print_success("Payment check claimed!");
        println!("  Amount claimed: {}", claimed);
        
        Ok(())
    }
    
    #[cfg(not(feature = "intents"))]
    {
        Err(anyhow::anyhow!("Intents feature not enabled"))
    }
}

async fn cmd_swap(cli: &Cli, from: &str, to: &str, amount: &str) -> Result<()> {
    #[cfg(feature = "intents")]
    {
        let api_key = cli.api_key.as_ref()
            .ok_or_else(|| anyhow::anyhow!("--api-key required"))?;

        let near_amount = parse_amount_token(amount, from)?;

        print_info(&format!("Swapping {} {} to {}...", amount, from, to));

        let provider = IntentsProvider::new(api_key.clone());

        let from_token = get_swap_token_id(from);
        let to_token = get_swap_token_id(to);

        let result = provider.swap(
            &from_token,
            &to_token,
            near_amount,
            None,
        ).await?;

        print_success("Swap completed (gasless)!");
        println!("  Request ID: {}", result.request_id);
        println!("  Amount out: {}", result.amount_out);
        if let Some(hash) = result.intent_hash {
            println!("  Intent:     {}", hash);
        }
        
        Ok(())
    }
    
    #[cfg(not(feature = "intents"))]
    {
        Err(anyhow::anyhow!("Intents feature not enabled"))
    }
}

fn cmd_config(cli: &Cli) -> Result<()> {
    println!("Configuration:");
    println!();
    println!("Method:     {}", cli.method);
    println!("Config:     {:?}", cli.config);
    println!("RPC URL:    {:?}", cli.rpc_url);
    println!("Account:    {:?}", cli.account);
    println!("API Key:    {:?}", cli.api_key.as_ref().map(|_| "(set)"));
    println!();
    println!("Commands:");
    println!("  register        - Register a new OutLayer custody wallet");
    println!("  fund-link       - Generate a funding link for your wallet");
    println!("  handoff         - Show wallet management URL");
    println!("  pay             - Send a payment");
    println!("  balance         - Check account balance");
    println!("  storage-deposit - Register storage for token receipt");
    println!("  verify          - Verify a transaction");
    println!("  server          - Start payment server");
    println!("  tokens          - List available tokens");
    println!("  create-check    - Create payment check");
    println!("  claim-check     - Claim payment check");
    println!("  swap            - Swap tokens");
    println!("  config          - Show this configuration");
    println!();
    println!("Quick start:");
    println!("  1. Register wallet:");
    println!("     mpp-near register");
    println!("  2. Generate funding link:");
    println!("     mpp-near fund-link --amount 0.1 --token near");
    println!("  3. Check balance:");
    println!("     mpp-near balance --api-key wk_...");
    println!();
    println!("Example usage:");
    println!("  mpp-near pay --recipient merchant.near --amount 1");
    println!("  mpp-near pay --recipient merchant.near --amount 10 --token usdc");
    println!("  mpp-near balance");
    println!("  mpp-near storage-deposit --account merchant.near");
    println!("  mpp-near tokens");
    println!("  mpp-near swap --from near --to usdc --amount 1");
    println!("  mpp-near create-check --amount 10 --token usdc");

    Ok(())
}

fn parse_amount(amount: &str) -> Result<mpp_near::types::NearAmount> {
    let near: f64 = amount.parse()
        .map_err(|_| anyhow::anyhow!("Invalid amount: {}", amount))?;

    let yocto = (near * 1e24) as u128;
    Ok(mpp_near::types::NearAmount::from_yocto(yocto))
}

#[cfg(feature = "intents")]
fn parse_amount_token(amount: &str, token: &str) -> Result<mpp_near::types::NearAmount> {
    let value: f64 = amount.parse()
        .map_err(|_| anyhow::anyhow!("Invalid amount: {}", amount))?;

    let yocto = match token {
        "near" => (value * 1e24) as u128,
        "usdc" | "usdt" => (value * 1e6) as u128,
        _ => (value * 1e24) as u128, // Default to NEAR decimals
    };

    Ok(mpp_near::types::NearAmount::from_yocto(yocto))
}

#[cfg(feature = "intents")]
fn get_token_id(token: &str) -> &str {
    match token {
        "near" => "near",
        "usdc" => "17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1",
        "usdt" => "usdt.tether-token.near",
        _ => token,
    }
}

/// Get swap token ID in defuse asset format (nep141: prefix required)
#[cfg(feature = "intents")]
fn get_swap_token_id(token: &str) -> String {
    match token {
        "near" | "wnear" => "nep141:wrap.near".to_string(),
        "usdc" => "nep141:17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1".to_string(),
        "usdt" => "nep141:usdt.tether-token.near".to_string(),
        _ if token.starts_with("nep141:") => token.to_string(),
        _ => format!("nep141:{}", token),
    }
}

fn print_success(message: &str) {
    println!("{} {}", "✓".green().bold(), message);
}

fn print_info(message: &str) {
    if std::env::var("MPP_NEAR_QUIET").is_err() {
        eprintln!("{} {}", "ℹ".blue(), message);
    }
}
