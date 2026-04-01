use serde_json::{json, Value};
use wasi_http_client::Client;

pub fn view_call(contract_id: &str, method_name: &str, args_b64: &str) -> Result<Value, String> {
    let body = json!({
        "jsonrpc": "2.0",
        "id": "dontcare",
        "method": "query",
        "params": {
            "request_type": "call_function",
            "finality": "final",
            "account_id": contract_id,
            "method_name": method_name,
            "args_base64": args_b64,
        }
    });

    let resp = Client::new()
        .post("https://rpc.mainnet.near.org")
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .map_err(|e| format!("HTTP error: {:?}", e))?;

    let data: Value = serde_json::from_slice(&resp.body().map_err(|e| format!("body: {}", e))?)
        .map_err(|e| format!("JSON parse error: {}", e))?;

    Ok(data)
}
