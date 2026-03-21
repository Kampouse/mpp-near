//! Tool to check access keys for a NEAR account

use clap::Parser;
use near_jsonrpc_client::JsonRpcClient;
use near_jsonrpc_client::methods::query::RpcQueryRequest;
use near_primitives::views::QueryRequest;
use near_primitives::types::{AccountId, BlockReference};

#[derive(Parser)]
struct Cli {
    account_id: String,
    #[arg(short, long, default_value = "https://rpc.testnet.near.org")]
    rpc_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let client = JsonRpcClient::connect(&cli.rpc_url);

    println!("Checking access keys for: {}", cli.account_id);
    println!();

    let account_id: AccountId = cli.account_id.parse()
        .map_err(|_| "Invalid account ID")?;

    // Query all access keys
    let request = QueryRequest::ViewAccessKeyList {
        account_id: account_id.clone(),
    };

    let query_request = RpcQueryRequest {
        request,
        block_reference: BlockReference::latest(),
    };

    match client.call(query_request).await {
        Ok(response) => {
            if let Ok(value) = serde_json::to_string_pretty(&response) {
                println!("{}", value);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("\nThis could mean:");
            eprintln!("  1. Account doesn't exist on this network");
            eprintln!("  2. RPC URL is incorrect");
        }
    }

    Ok(())
}
