// src/backtest.rs
use std::env;

/// Returns true if the bot is running in dry-run mode.
pub fn is_dry_run() -> bool {
    env::var("DRY_RUN")
        .unwrap_or_else(|_| "true".to_string())
        .to_lowercase()
        == "true"
}
