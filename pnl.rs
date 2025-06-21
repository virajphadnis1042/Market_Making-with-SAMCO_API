use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicI64, Ordering};

static TOTAL_PNL: Lazy<AtomicI64> = Lazy::new(|| AtomicI64::new(0));

pub fn update_pnl(delta: f64) {
    let delta_i64 = (delta * 100.0).round() as i64;
    TOTAL_PNL.fetch_add(delta_i64, Ordering::Relaxed);
}

pub fn get_total_pnl() -> f64 {
    TOTAL_PNL.load(Ordering::Relaxed) as f64 / 100.0
}

pub fn record_fill(_symbol: &str, price: f64, side: &str, qty: u32, hedge: bool) {
    let direction = match side {
        "BUY" => -1.0,
        "SELL" => 1.0,
        _ => 0.0,
    };

    let delta = direction * price * qty as f64;
    update_pnl(delta);

    if !hedge {
        println!(
            "💸 Spread fill: {} {} @ ₹{:.2} → Δ PnL: {:.2}",
            side, qty, price, delta
        );
    }
}
