// src/config.rs
use std::env;

/// Returns `true` if DRY_RUN mode is enabled from the environment.
pub fn dry_run_enabled() -> bool {
    env::var("DRY_RUN")
        .unwrap_or_else(|_| "true".to_string())
        .to_lowercase()
        == "true"
}

/// Reads a required configuration key or panics if missing.
pub fn get_required_var(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("Missing required env var: {}", key))
}
