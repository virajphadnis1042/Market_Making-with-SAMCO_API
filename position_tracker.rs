use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;

/// Fetches all current positions from Samco and returns symbol → net qty
pub async fn fetch_positions(session_token: &str) -> Result<HashMap<String, i32>, reqwest::Error> {
    let client = Client::new();
    let url = "https://tradeapi.samco.in/position/getPositions?positionType=NET";

    let res = client
        .get(url)
        .header("x-session-token", session_token)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json::<Value>()
        .await?;

    let mut positions = HashMap::new();

    if let Some(array) = res["positionDetails"].as_array() {
        for item in array {
            let symbol = item["tradingSymbol"].as_str().unwrap_or("").to_string();
            let qty = item["netQuantity"].as_str()
                .and_then(|s| s.parse::<i32>().ok())
                .unwrap_or(0);
            positions.insert(symbol, qty);
        }
    }

    Ok(positions)
}
