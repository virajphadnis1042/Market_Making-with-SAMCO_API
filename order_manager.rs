use reqwest::Client;
use serde_json::{json, Value};
use crate::strategy::Quote;

pub async fn place_quote_orders(
    session_token: &str,
    quote: &Quote,
    lot_size: u32,
) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let exchange = "NSE";

    let buy_order = json!({
        "orderType": "LIMIT",
        "productType": "INTRADAY",
        "exchange": exchange,
        "symbolName": quote.symbol,
        "quantity": lot_size.to_string(),
        "disclosedQuantity": "0",
        "price": quote.bid_price.to_string(),
        "triggerPrice": "0",
        "orderValidity": "DAY",
        "variety": "NORMAL",
        "transactionType": "BUY"
    });

    let sell_order = json!({
        "orderType": "LIMIT",
        "productType": "INTRADAY",
        "exchange": exchange,
        "symbolName": quote.symbol,
        "quantity": lot_size.to_string(),
        "disclosedQuantity": "0",
        "price": quote.ask_price.to_string(),
        "triggerPrice": "0",
        "orderValidity": "DAY",
        "variety": "NORMAL",
        "transactionType": "SELL"
    });

    // Place BUY
    let res1 = client
        .post("https://tradeapi.samco.in/order/placeOrder")
        .header("x-session-token", session_token)
        .header("Content-Type", "application/json")
        .json(&buy_order)
        .send()
        .await?
        .json::<Value>()
        .await?;

    println!("📤 BUY order: {:?}", res1);

    // Place SELL
    let res2 = client
        .post("https://tradeapi.samco.in/order/placeOrder")
        .header("x-session-token", session_token)
        .header("Content-Type", "application/json")
        .json(&sell_order)
        .send()
        .await?
        .json::<Value>()
        .await?;

    println!("📤 SELL order: {:?}", res2);

    Ok(())
}
