use reqwest::Client;
use serde_json::Value;

/// Fetch ATM CE and PE symbols for NIFTY using Samco optionChain API
pub async fn get_atm_nifty_option_symbols(session_token: &str) -> Result<Vec<(String, f64)>, reqwest::Error> {
    let client = Client::new();

    let url = "https://tradeapi.samco.in/option/optionChain?searchSymbolName=NIFTY";

    let response = client
        .get(url)
        .header("x-session-token", session_token)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json::<Value>()
        .await?;

    // Get spot price
    let spot = response["underlyingValue"]
        .as_str()
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0);

    let atm_strike = (spot / 50.0).round() * 50.0;

    let mut results = vec![];

    if let Some(chain) = response["optionChain"].as_array() {
        for option in chain {
            let strike = option["strikePrice"].as_str()
                .and_then(|s| s.parse::<f64>().ok())
                .unwrap_or(0.0);

            let opt_type = option["optionType"].as_str().unwrap_or("");
            let symbol = option["tradingSymbol"].as_str().unwrap_or("");
            let ltp = option["lastTradedPrice"].as_str()
                .and_then(|s| s.parse::<f64>().ok())
                .unwrap_or(0.0);

            if strike == atm_strike && (opt_type == "CE" || opt_type == "PE") {
                results.push((symbol.to_string(), ltp));
            }
        }
    }

    Ok(results)
}
