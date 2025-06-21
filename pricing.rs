// Fully implemented pricing.rs module
// src/pricing.rs
use statrs::distribution::{ContinuousCDF, Normal};

/// Calculates the Black-Scholes price of a call or put option.
pub fn black_scholes(
    spot: f64,
    strike: f64,
    time_to_expiry: f64, // in years
    volatility: f64,
    risk_free_rate: f64,
    is_call: bool,
) -> f64 {
    let norm = Normal::new(0.0, 1.0).unwrap();

    let d1 = ((spot / strike).ln() + (risk_free_rate + 0.5 * volatility.powi(2)) * time_to_expiry)
        / (volatility * time_to_expiry.sqrt());
    let d2 = d1 - volatility * time_to_expiry.sqrt();

    if is_call {
        spot * norm.cdf(d1) - strike * (-risk_free_rate * time_to_expiry).exp() * norm.cdf(d2)
    } else {
        strike * (-risk_free_rate * time_to_expiry).exp() * norm.cdf(-d2) - spot * norm.cdf(-d1)
    }
}

/// Simple wrapper to compute fair value for quoting.
pub fn compute_fair_price(
    spot: f64,
    strike: f64,
    time_to_expiry_days: f64,
    volatility: f64,
    is_call: bool,
) -> f64 {
    let time_to_expiry = time_to_expiry_days / 365.0;
    let r = 0.06; // assumed risk-free rate
    black_scholes(spot, strike, time_to_expiry, volatility, r, is_call)
}
