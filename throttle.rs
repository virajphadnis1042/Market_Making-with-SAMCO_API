// Fully implemented throttle.rs module
// src/throttle.rs
use tokio::time::{sleep, Duration};

/// Delays each loop iteration to avoid hitting API rate limits.
/// Default delay: 1000ms (1 second)
pub async fn rate_limit() {
    let delay_ms = std::env::var("THROTTLE_MS")
        .unwrap_or_else(|_| "1000".to_string())
        .parse::<u64>()
        .unwrap_or(1000);

    sleep(Duration::from_millis(delay_ms)).await;
}
