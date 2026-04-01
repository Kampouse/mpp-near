use serde_json::{json, Value};
use wasi_http_client::Client;

use crate::tokens::Token;

pub fn execute_swap(token_in: &Token, token_out: &Token, amount: &str) -> Result<Value, String> {
    let api_key = get_api_key()?;

    let body = json!({
        "token_in": token_in.defuse_id(),
        "token_out": token_out.defuse_id(),
        "amount_in": amount,
    });

    let resp = Client::new()
        .post("https://api.outlayer.fastnear.com/wallet/v1/intents/swap")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&body)
        .send()
        .map_err(|e| format!("HTTP error: {:?}", e))?;

    let data: Value = serde_json::from_slice(&resp.body().map_err(|e| format!("body: {}", e))?)
        .map_err(|e| format!("JSON parse error: {}", e))?;

    Ok(json!({"success": true, "data": data}))
}

fn get_api_key() -> Result<String, String> {
    std::env::var("OUTLAYER_API_KEY").map_err(|_| "OUTLAYER_API_KEY not set".into())
}
