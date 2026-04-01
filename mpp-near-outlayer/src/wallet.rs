use serde_json::{json, Value};
use wasi_http_client::Client;

use crate::tokens::Token;

const BASE_URL: &str = "https://api.outlayer.fastnear.com";

fn get_api_key() -> Result<String, String> {
    std::env::var("OUTLAYER_API_KEY").map_err(|_| "OUTLAYER_API_KEY not set".into())
}

pub fn get_balances() -> Result<Value, String> {
    let api_key = get_api_key()?;

    let resp = Client::new()
        .get(&format!("{}/wallet/v1/intents/balances", BASE_URL))
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .map_err(|e| format!("HTTP error: {:?}", e))?;

    let data: Value = serde_json::from_slice(&resp.body().map_err(|e| format!("body: {}", e))?)
        .map_err(|e| format!("JSON parse error: {}", e))?;

    Ok(data)
}

pub fn send_payment(token: &Token, amount: &str, receiver: &str) -> Result<Value, String> {
    let api_key = get_api_key()?;

    let body = json!({
        "token": token.defuse_id(),
        "amount": amount,
        "receiver": receiver,
    });

    let resp = Client::new()
        .post(&format!("{}/wallet/v1/intents/transfer", BASE_URL))
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&body)
        .send()
        .map_err(|e| format!("HTTP error: {:?}", e))?;

    let data: Value = serde_json::from_slice(&resp.body().map_err(|e| format!("body: {}", e))?)
        .map_err(|e| format!("JSON parse error: {}", e))?;

    Ok(data)
}
