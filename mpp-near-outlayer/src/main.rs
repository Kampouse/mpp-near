mod tokens;
mod swap;
mod wallet;
mod mpp;
mod rpc;

use serde_json::{json, Value};
use std::io::{self, Read, Write};
use tokens::Token;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let result = match handle(&input) {
        Ok(v) => v,
        Err(e) => json!({"error": e}),
    };

    let stdout = io::stdout();
    let mut out = stdout.lock();
    writeln!(out, "{}", serde_json::to_string(&result).unwrap()).unwrap();
}

fn handle(input: &str) -> Result<Value, String> {
    let req: Value = serde_json::from_str(input).map_err(|e| format!("Invalid JSON: {}", e))?;
    let action = req.get("action").and_then(|v| v.as_str()).ok_or("Missing 'action'")?;

    match action {
        "prices" => fetch_prices(),
        "swap" => {
            let token_in = req.get("token_in").and_then(|v| v.as_str()).ok_or("Missing token_in")?;
            let token_out = req.get("token_out").and_then(|v| v.as_str()).ok_or("Missing token_out")?;
            let amount = req.get("amount").and_then(|v| v.as_str()).ok_or("Missing amount")?;
            let tin = Token::from_name(token_in).ok_or("Unknown token_in")?;
            let tout = Token::from_name(token_out).ok_or("Unknown token_out")?;
            swap::execute_swap(&tin, &tout, amount)
        }
        "balance" => wallet::get_balances(),
        "pay" => {
            let token = req.get("token").and_then(|v| v.as_str()).ok_or("Missing token")?;
            let amount = req.get("amount").and_then(|v| v.as_str()).ok_or("Missing amount")?;
            let receiver = req.get("receiver").and_then(|v| v.as_str()).ok_or("Missing receiver")?;
            let t = Token::from_name(token).ok_or("Unknown token")?;
            wallet::send_payment(&t, amount, receiver)
        }
        "verify" => {
            let challenge = req.get("challenge").ok_or("Missing challenge")?;
            let credential = req.get("credential").ok_or("Missing credential")?;
            mpp::verify_mpp(challenge, credential)
        }
        "rpc_view" => {
            let contract_id = req.get("contract_id").and_then(|v| v.as_str()).ok_or("Missing contract_id")?;
            let method_name = req.get("method_name").and_then(|v| v.as_str()).ok_or("Missing method_name")?;
            let args_b64 = req.get("args_base64").and_then(|v| v.as_str()).unwrap_or("");
            rpc::view_call(contract_id, method_name, args_b64)
        }
        _ => Err(format!("Unknown action: {}", action)),
    }
}

fn fetch_prices() -> Result<Value, String> {
    use wasi_http_client::Client;

    let resp = Client::new()
        .get("https://1click.chaindefuser.com/v0/tokens")
        .send()
        .map_err(|e| format!("HTTP error: {:?}", e))?;

    let data: Value = serde_json::from_slice(&resp.body().map_err(|e| format!("body: {}", e))?)
        .map_err(|e| format!("JSON parse error: {}", e))?;

    // Extract prices for our tokens
    let tokens_array = data.as_array().ok_or("Expected array response")?;
    let mut prices = serde_json::Map::new();

    for token_entry in tokens_array {
        let defuse_id = token_entry.get("defuse_account_id")
            .or_else(|| token_entry.get("id"))
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let price = token_entry.get("price")
            .and_then(|v| v.as_f64());

        if let Some(token) = Token::from_defuse_id(defuse_id) {
            if let Some(p) = price {
                prices.insert(format!("{:?}", token), json!(p));
            }
        }
    }

    Ok(json!({"prices": prices}))
}
