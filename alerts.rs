// Fully implemented alerts.rs module
// src/alerts.rs
use chrono::Local;

/// Sends an alert message (currently prints to stderr).
/// Can be extended to send via Telegram, Email, Discord, etc.
pub fn send_alert(message: &str) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    eprintln!("🚨 ALERT [{}] — {}", timestamp, message);
}
