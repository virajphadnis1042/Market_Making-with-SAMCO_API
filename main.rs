mod auth;
mod config;
mod market_data;
mod pricing;
mod strategy;
mod order_manager;
mod position_tracker;
mod hedging;
mod orderbook;
mod pnl;
mod greeks;
mod risk_engine;
mod logger;
mod monitor;
mod alerts;
mod backtest;
mod throttle;
mod symbol_selector;

use dotenv::dotenv;
use log::info;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    info!("🟢 Samco Market Making Bot starting...");

    let dry_run = backtest::is_dry_run();

    // ✅ Login to Samco
    let session_token = match auth::login().await {
        Ok(token) => token,
        Err(e) => {
            alerts::send_alert(&format!("❌ Login failed: {}", e));
            return;
        }
    };

    // ✅ Fetch live ATM CE/PE symbols
    let atm_symbols = match symbol_selector::get_atm_nifty_option_symbols(&session_token).await {
        Ok(symbols) => symbols,
        Err(e) => {
            println!("❌ Failed to fetch ATM symbols: {}", e);
            vec![]
        }
    };

    if atm_symbols.is_empty() {
        println!("⚠️ No ATM symbols found. Aborting...");
        return;
    }

    let symbols: Vec<&str> = atm_symbols.iter().map(|(s, _)| s.as_str()).collect();
    println!("✅ Selected ATM symbols: {:?}", symbols);

    let spread_percent = 0.5;
    let lot_size = 75;

    loop {
        if risk_engine::is_halted() {
            alerts::send_alert("🚨 Bot halted by risk engine.");
            break;
        }

        // ✅ Fetch quotes
        let quote_data = market_data::fetch_quotes(&session_token, &symbols).await;

        if let Err(e) = &quote_data {
            println!("❌ Quote fetch failed: {}", e);
            throttle::rate_limit().await;
            continue;
        }

        let quotes_json = quote_data.unwrap();
        let mut fair_quotes: Vec<(String, f64)> = vec![];

        for symbol in &symbols {
            let path = &quotes_json["multiQuotes"][*symbol]["lastTradedPrice"];
            if let Some(price_str) = path.as_str() {
                if let Ok(price) = price_str.parse::<f64>() {
                    fair_quotes.push((symbol.to_string(), price));
                } else {
                    println!("⚠️ Invalid price format for {}", symbol);
                }
            } else {
                println!("⚠️ LTP not found for {}", symbol);
            }
        }

        // ✅ Generate bid/ask quote prices
        let generated_quotes = strategy::generate_quotes(&fair_quotes, spread_percent);

        for quote in &generated_quotes {
            if dry_run {
                println!(
                    "💡 Simulated quote for {} → Bid: {:.2}, Ask: {:.2}",
                    quote.symbol, quote.bid_price, quote.ask_price
                );
                pnl::record_fill(&quote.symbol, quote.bid_price, "SELL", lot_size, false);
                pnl::record_fill(&quote.symbol, quote.ask_price, "BUY", lot_size, false);
            } else {
                match order_manager::place_quote_orders(&session_token, quote, lot_size).await {
                    Ok(_) => println!("✅ Orders placed for {}", quote.symbol),
                    Err(e) => println!("❌ Order error: {}", e),
                }
            }
        }

        // ✅ Fetch positions and compute net delta
        let positions = position_tracker::fetch_positions(&session_token)
            .await
            .unwrap_or_default();

        let mut net_delta = 0.0;
        for (_symbol, qty) in &positions {
            net_delta += *qty as f64; // You can weight by Greeks later
        }

        // ✅ Auto-hedging
        if !dry_run {
            let _ = hedging::auto_hedge(&session_token, net_delta).await;
            let _ = orderbook::check_and_cancel_stale_orders(&session_token).await;
        }

        // ✅ Dashboard + PnL
        monitor::print_status(net_delta);
        throttle::rate_limit().await;
    }

    println!("⛔ Bot shutdown complete.");
}
