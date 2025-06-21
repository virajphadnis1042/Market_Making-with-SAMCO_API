use reqwest::Client;
use serde_json::{json, Value};

/// Fetch live LTPs for multiple symbols via Samco multiQuote API
pub async fn fetch_quotes(session_token: &str, symbols: &[&str]) -> Result<Value, reqwest::Error> {
    let client = Client::new();

    let url = "https://tradeapi.samco.in/quote/multiQuote";
    let body = json!({ "symbolList": symbols });

    let response = client
        .post(url)
        .header("x-session-token", session_token)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?
        .json::<Value>()
        .await?;

    Ok(response)
}
