// Fully implemented strategy.rs module
// src/strategy.rs

/// Represents a bid/ask quote for a symbol.
pub struct Quote {
    pub symbol: String,
    pub bid_price: f64,
    pub ask_price: f64,
}

/// Generates bid and ask prices around the fair value using a configured spread.
/// For example, if fair = 100 and spread = 1%, this returns [99.0, 101.0].
pub fn generate_quote(symbol: &str, fair_price: f64, spread_percent: f64) -> Quote {
    let spread = fair_price * (spread_percent / 100.0);
    let bid = fair_price - spread;
    let ask = fair_price + spread;

    Quote {
        symbol: symbol.to_string(),
        bid_price: bid,
        ask_price: ask,
    }
}

/// Generates a list of quotes for multiple instruments.
pub fn generate_quotes(
    fair_prices: &[(String, f64)],
    spread_percent: f64,
) -> Vec<Quote> {
    fair_prices
        .iter()
        .map(|(symbol, fair)| generate_quote(symbol, *fair, spread_percent))
        .collect()
}
