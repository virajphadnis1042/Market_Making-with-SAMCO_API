// Fully implemented logger.rs module
// src/logger.rs
use chrono::Local;
use std::fs::{OpenOptions};
use std::io::Write;
use std::path::Path;

/// Logs a trade to a CSV file
pub fn log_trade(symbol: &str, side: &str, price: f64, quantity: u32, is_hedge: bool) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let log_line = format!("{},{},{},{},{},{}\n", timestamp, symbol, side, price, quantity, is_hedge);

    let file_path = Path::new("trades_log.csv");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .expect("Unable to open or create log file");

    file.write_all(log_line.as_bytes()).expect("Unable to write log line");
}
