//! Main CLI entry point

use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use mpp_near::types::AccountId;
use std::path::PathBuf;

#[cfg(feature = "client")]
use mpp_near::client::NearProvider;

#[cfg(feature = "intents")]
use mpp_near::client::IntentsProvider;

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
        #[arg(short, long)]
        amount: String,

        /// Token to send (near, usdc, usdt)
        #[arg(short = 't', long, default_value = "near")]
        token: String,

        /// Memo to include with transaction
        #[arg(short, long)]
        memo: Option<String>,
    },

    /// Check account balance
    Balance {
        /// Account to check (defaults to configured account)
        #[arg(short, long)]
        account: Option<String>,
    },

    /// List available tokens (intents only)
    Tokens,

    /// Show current configuration
    Config,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

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
        Commands::Pay { recipient, amount, token, memo } => {
            cmd_pay(&cli, recipient, amount, token, memo.as_deref()).await?;
        }

        Commands::Balance { account } => {
            cmd_balance(&cli, account.as_deref()).await?;
        }

        Commands::Tokens => {
            cmd_tokens(&cli).await?;
        }

        Commands::Config => {
            cmd_config(&cli)?;
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

                let tx_hash = match token {
                    "near" => provider.transfer(&recipient, near_amount).await?,
                    "usdc" => provider.transfer_token(
                        "17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1",
                        &recipient,
                        near_amount,
                    ).await?,
                    _ => return Err(anyhow::anyhow!("Unsupported token: {}", token)),
                };

                print_success("Payment sent (gasless)!");
                println!("  Transaction: {}", tx_hash);
                println!("  Recipient:   {}", recipient);
                println!("  Amount:      {} {}", near_amount, token);
                println!("  Gas cost:    0 (paid by solver)");
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

async fn cmd_tokens(cli: &Cli) -> Result<()> {
    #[cfg(feature = "intents")]
    {
        let api_key = cli.api_key.as_ref()
            .ok_or_else(|| anyhow::anyhow!("--api-key required"))?;

        print_info("Fetching available tokens...");

        let provider = IntentsProvider::new(api_key.clone());
        let tokens = provider.list_tokens().await?;

        print_success(&format!("Found {} tokens", tokens.len()));

        for token in tokens.iter().take(20) {
            println!("  {:6} - {} ({})", token.symbol, token.name, token.chain);
        }

        if tokens.len() > 20 {
            println!("  ... and {} more", tokens.len() - 20);
        }
    }

    #[cfg(not(feature = "intents"))]
    return Err(anyhow::anyhow!("Intents feature not enabled"));

    Ok(())
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
    println!("Example usage:");
    println!("  mpp-near pay --recipient merchant.near --amount 1");
    println!("  mpp-near pay --recipient merchant.near --amount 10 --token usdc");
    println!("  mpp-near balance");
    println!("  mpp-near tokens");

    Ok(())
}

fn parse_amount(amount: &str) -> Result<mpp_near::types::NearAmount> {
    let near: f64 = amount.parse()
        .map_err(|_| anyhow::anyhow!("Invalid amount: {}", amount))?;

    let yocto = (near * 1e24) as u128;
    Ok(mpp_near::types::NearAmount::from_yocto(yocto))
}

fn print_success(message: &str) {
    println!("{} {}", "✓".green().bold(), message);
}

fn print_info(message: &str) {
    if std::env::var("MPP_NEAR_QUIET").is_err() {
        eprintln!("{} {}", "ℹ".blue(), message);
    }
}
