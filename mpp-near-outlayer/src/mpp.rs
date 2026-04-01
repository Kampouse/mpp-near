use serde_json::{json, Value};

/// Verify an MPP challenge/credential JSON.
/// Checks: nonce match, realm match, method check.
pub fn verify_mpp(challenge: &Value, credential: &Value) -> Result<Value, String> {
    let challenge_nonce = challenge.get("nonce")
        .and_then(|v| v.as_str())
        .ok_or("challenge missing nonce")?;
    let challenge_realm = challenge.get("realm")
        .and_then(|v| v.as_str())
        .ok_or("challenge missing realm")?;
    let challenge_method = challenge.get("method")
        .and_then(|v| v.as_str())
        .ok_or("challenge missing method")?;

    let cred_nonce = credential.get("nonce")
        .and_then(|v| v.as_str())
        .ok_or("credential missing nonce")?;
    let cred_realm = credential.get("realm")
        .and_then(|v| v.as_str())
        .ok_or("credential missing realm")?;
    let cred_method = credential.get("method")
        .and_then(|v| v.as_str())
        .ok_or("credential missing method")?;

    let nonce_ok = challenge_nonce == cred_nonce;
    let realm_ok = challenge_realm == cred_realm;
    let method_ok = challenge_method == cred_method;

    let valid = nonce_ok && realm_ok && method_ok;

    Ok(json!({
        "valid": valid,
        "nonce_match": nonce_ok,
        "realm_match": realm_ok,
        "method_match": method_ok,
    }))
}
