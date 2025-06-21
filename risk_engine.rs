// Fully implemented risk_engine.rs module
// src/risk_engine.rs
use crate::pnl::get_total_pnl;
use std::sync::atomic::{AtomicBool, Ordering};
use once_cell::sync::Lazy;

/// Max loss threshold (adjust as needed)
const MAX_DAILY_LOSS: f64 = -5000.0;
const MAX_DELTA_EXPOSURE: f64 = 50000.0;

static BOT_HALTED: Lazy<AtomicBool> = Lazy::new(|| AtomicBool::new(false));

/// Checks if bot should stop trading due to excessive loss or exposure.
pub fn check_risk(net_delta: f64) -> bool {
    let current_pnl = get_total_pnl();

    if current_pnl <= MAX_DAILY_LOSS {
        eprintln!("🚨 Max daily loss exceeded: ₹{:.2}", current_pnl);
        BOT_HALTED.store(true, Ordering::Relaxed);
        return true;
    }

    if net_delta.abs() > MAX_DELTA_EXPOSURE {
        eprintln!("⚠️ Delta exposure too high: {}", net_delta);
        return true;
    }

    false
}

/// Returns true if the bot is halted due to risk breach.
pub fn is_halted() -> bool {
    BOT_HALTED.load(Ordering::Relaxed)
}
