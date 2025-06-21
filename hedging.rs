use reqwest::Client;
use serde_json::json;

/// Automatically hedge using NIFTY futures based on net delta exposure
pub async fn auto_hedge(session_token: &str, net_delta: f64) -> Result<(), reqwest::Error> {
    let hedge_threshold = 75.0; // only hedge if delta exceeds 1 lot
    let lot_size = 75;
    let hedge_symbol = "NIFTY24JUNFUT"; // can be made dynamic

    if net_delta.abs() < hedge_threshold {
        println!("✅ Net delta ({:.2}) within hedge band. No hedge needed.", net_delta);
        return Ok(());
    }

    let hedge_side = if net_delta > 0.0 { "SELL" } else { "BUY" };
    let hedge_qty = lot_size;

    // Dummy hedge price (replace with LTP logic later)
    let hedge_price = if hedge_side == "BUY" { 22150.0 } else { 22155.0 };

    let hedge_order = json!({
        "orderType": "LIMIT",
        "productType": "INTRADAY",
        "exchange": "NSE",
        "symbolName": hedge_symbol,
        "quantity": hedge_qty.to_string(),
        "disclosedQuantity": "0",
        "price": hedge_price.to_string(),
        "triggerPrice": "0",
        "orderValidity": "DAY",
        "variety": "NORMAL",
        "transactionType": hedge_side
    });

    let client = Client::new();
    let res = client
        .post("https://tradeapi.samco.in/order/placeOrder")
        .header("x-session-token", session_token)
        .header("Content-Type", "application/json")
        .json(&hedge_order)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    println!("⚖️ Hedge order placed: {} {} @ ₹{:.2}", hedge_side, hedge_symbol, hedge_price);
    println!("📥 Samco Response: {:?}", res);

    Ok(())
}
