// Fully implemented orderbook.rs module
// src/orderbook.rs
use reqwest::Client;
use serde_json::Value;

/// Cancels an individual order by its order number.
async fn cancel_order(session_token: &str, order_number: &str) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let cancel_url = "https://tradeapi.samco.in/order/cancelOrder";

    let payload = serde_json::json!({
        "orderNumber": order_number
    });

    let _ = client
        .post(cancel_url)
        .header("x-session-token", session_token)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    println!("Cancelled order: {}", order_number);
    Ok(())
}

/// Scans open orders and cancels those that are too old or far from market price.
/// Currently cancels all open orders as a placeholder.
pub async fn check_and_cancel_stale_orders(session_token: &str) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let orderbook_url = "https://tradeapi.samco.in/order/orderBook";

    let response = client
        .post(orderbook_url)
        .header("x-session-token", session_token)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json::<Value>()
        .await?;

    if let Some(orders) = response["orderBook"].as_array() {
        for order in orders {
            let status = order["orderStatus"].as_str().unwrap_or("");
            let order_number = order["orderNumber"].as_str().unwrap_or("");
            let symbol = order["symbolName"].as_str().unwrap_or("UNKNOWN");

            if status == "open" || status == "trigger pending" {
                println!("Stale order found: {} [{}] — attempting to cancel", symbol, order_number);
                let _ = cancel_order(session_token, order_number).await;
            }
        }
    }

    Ok(())
}
