wit_bindgen::generate!({
    world: "tool",
    exports: { Tool: Guest },
});

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

// ─── Token mappings ───

fn token_defuse_id(symbol: &str) -> &'static str {
    match symbol {
        "USDC" => "nep141:17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1",
        "ZEC" => "nep141:zec.omft.near",
        "BTC" => "nep141:btc.omft.near",
        "ETH" => "nep141:eth.omft.near",
        "SOL" => "nep141:sol.omft.near",
        "NEAR" | "WNEAR" => "nep141:wrap.near",
        _ => return Err(format!("unknown token: {}", symbol)),
    }
}

fn token_decimals(symbol: &str) -> u32 {
    match symbol {
        "USDC" => 6,
        "ZEC" | "BTC" => 8,
        "ETH" => 18,
        "SOL" => 9,
        "NEAR" | "WNEAR" => 24,
        _ => 18,
    }
}

fn to_raw_amount(token: &str, amount: f64) -> u128 {
    let dec = token_decimals(token) as u32;
    (amount * 10f64.powi(dec as i32)) as u128
}

// ─── HTTP helpers using IronClaw host functions ───

fn http_get(url: &str) -> Result<Vec<u8>, String> {
    let resp = host::http_request(
        "GET".to_string(),
        url.to_string(),
        "{}".to_string(),
        None,
        Some(30_000u32),
    ).map_err(|e| format!("HTTP GET error: {}", e))?;
    Ok(resp.body)
}

fn http_post(url: &str, body: Vec<u8>, auth: Option<&str>) -> Result<Vec<u8>, String> {
    let headers = if let Some(a) = auth {
        format!(r#"{{"Authorization":"Bearer {}","Content-Type":"application/json"}}"#, a)
    } else {
        r#"{"Content-Type":"application/json"}}"#
    };
    let resp = host::http_request(
        "POST".to_string(),
        url.to_string(),
        headers.to_string(),
        Some(&body),
        Some(30_000u32),
    ).map_err(|e| format!("HTTP POST error: {}", e))?;
    Ok(resp.body)
}

// ─── Action handlers ───

fn handle_prices() -> Result<Value, String> {
    let body = http_get("https://1click.chaindefuser.com/v0/tokens")?;
    let tokens: Vec<Value> = serde_json::from_slice(&body)
        .map_err(|e| format!("JSON parse error: {}", e))?;
    
    let tracked = ["ZEC", "BTC", "ETH", "SOL", "NEAR", "USDC"];
    let mut prices = serde_json::Map::new();
    if let Some(arr) = tokens.as_array() {
        for token in arr {
            let sym = token.get("symbol").and_then(|s| s.as_str()).unwrap_or("").to_uppercase();
            if tracked.contains(&sym.as_str()) {
                if let Some(price) = token.get("price").and_then(|p| p.as_f64()) {
                    prices.insert(sym.clone(), json!(price));
                }
            }
        }
    }
    Ok(json!(prices))
}

fn handle_swap(params: &Value) -> Result<Value, String> {
    let from = params["from"].as_str().ok_or("Missing 'from'")?;
    let to = params["to"].as_str().ok_or("Missing 'to'")?;
    let amount = params["amount"].as_f64().ok_or("Missing 'amount'")?;
    
    let from_token = token_defuse_id(from)?;
    let to_token = token_defuse_id(to)?;
    let raw_amount = to_raw_amount(from, amount);
    
    let payload = json!({
        "token_in": from_token,
        "token_out": to_token,
        "amount_in": raw_amount,
    });
    let body = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
    
    let result = http_post(
        "https://api.outlayer.fastnear.com/wallet/v1/intents/swap",
        body,
        None,
    )?;
    
    Ok(json!({
        "action": "swap",
        "from": from,
        "to": to,
        "amount": amount,
        "raw_amount": raw_amount,
        "result": serde_json::from_slice(&result).unwrap_or(json!({})),
    }))
}

fn handle_balance() -> Result<Value, String> {
    let body = http_get(
        "https://api.outlayer.fastnear.com/wallet/v1/intents/balances",
        None,
    )?;
    Ok(serde_json::from_slice(&body).unwrap_or(json!({})))
}

fn handle_pay(params: &Value) -> Result<Value, String> {
    let recipient = params["recipient"].as_str().ok_or("Missing 'recipient'")?;
    let amount = params["amount"].as_f64().ok_or("Missing 'amount'")?;
    let token = params["token"].as_str().unwrap_or("USDC");
    
    let token_id = token_defuse_id(token)?;
    let raw_amount = to_raw_amount(token, amount);
    
    let payload = json!({
        "receiver_id": recipient,
        "token_id": token_id,
        "amount": raw_amount,
    });
    let body = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
    
    let result = http_post(
        "https://api.outlayer.fastnear.com/wallet/v1/intents/transfer",
        body,
        None,
    )?;
    
    Ok(json!({
        "action": "pay",
        "recipient": recipient,
        "amount": amount,
        "token": token,
        "result": serde_json::from_slice(&result).unwrap_or(json!({})),
    }))
}

fn handle_verify(params: &Value) -> Result<Value, String> {
    let challenge_str = params["challenge"].as_str().ok_or("Missing 'challenge'")?;
    let credential_str = params["credential"].as_str().ok_or("Missing 'credential'")?;
    
    let challenge: Value = serde_json::from_str(challenge_str)
        .map_err(|e| format!("Invalid challenge JSON: {}", e))?;
    let credential: Value = serde_json::from_str(credential_str)
        .map_err(|e| format!("Invalid credential JSON: {}", e))?;
    
    let c_nonce = challenge["nonce"].as_str().unwrap_or("");
    let cr_nonce = credential["nonce"].as_str().unwrap_or("");
    
    if c_nonce.is_empty() {
        return Ok(json!({"valid": false, "reason": "Empty nonce"}));
    }
    if c_nonce != cr_nonce {
        return Ok(json!({"valid": false, "reason": "Nonce mismatch"}));
    }
    
    let c_realm = challenge["realm"].as_str().unwrap_or("");
    let cr_realm = credential["realm"].as_str().unwrap_or("");
    if c_realm != cr_realm {
        return Ok(json!({"valid": false, "reason": "Realm mismatch"}));
    }
    
    let c_method = challenge["method"].as_str().unwrap_or("");
    if !c_method.is_empty() && c_method != "near-intents" {
        return Ok(json!({"valid": false, "reason": "Unsupported method"}));
    }
    
    Ok(json!({"valid": true, "method": "near-intents", "realm": c_realm, "nonce": c_nonce}))
}

// ─── IronClaw Tool interface ───

struct MppNearTool;

impl Guest for MppNearTool {
    fn execute(req: Request) -> Response {
        let params: Value = match serde_json::from_str(&req.params) {
            Ok(p) => Response {
                output: Some(serde_json::to_string(&p).unwrap_or_default()),
                error: Some("Invalid params JSON".to_string()),
            },
            Err(e) => Response {
                output: None,
                error: Some(format!("Parse error: {}", e)),
            },
        };
        
        let result = match params.get("action").and_then(|a| a.as_str()) {
            Some("prices") => handle_prices(),
            Some("swap") => handle_swap(&params),
            Some("balance") => handle_balance(),
            Some("pay") => handle_pay(&params),
            Some("verify") => handle_verify(&params),
            _ => Err(format!("Unknown action: {}", a)),
        };
        
        match result {
            Ok(val) => Response {
                output: Some(serde_json::to_string(&json!({"status": "ok", "result": val}))),
                error: None,
            },
            Err(e) => Response {
                output: None,
                error: Some(e),
            },
        }
    }
    
    fn schema() -> String {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["prices", "swap", "balance", "pay", "verify"],
                },
                "from": {"type": "string", "description": "Source token (for swap)"},
                "to": {"type": "string", "description": "Target token (for swap)"},
                "amount": {"type": "number", "description": "Amount to swap/pay"},
                "recipient": {"type": "string", "description": "Recipient address (for pay)"},
                "token": {"type": "string", "description": "Token symbol (for pay)"},
                "challenge": {"type": "string", "description": "Challenge JSON (for verify)"},
                "credential": {"type": "string", "description": "Credential JSON (for verify)"}
            },
            "required": ["action"]
        }).to_string()
    }
    
    fn description() -> String {
        "MPP-NEAR: Cross-chain swap, balance, payment, and MPP verification tool for NEAR Protocol. Supports USDC, ZEC, BTC, ETH, SOL, NEAR tokens via OutLayer Intents API.".to_string()
    }
}
