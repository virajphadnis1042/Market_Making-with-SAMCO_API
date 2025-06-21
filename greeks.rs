// Fully implemented greeks.rs module
// src/greeks.rs
use statrs::distribution::{ContinuousCDF, Normal};
use statrs::distribution::Continuous; // REQUIRED for .pdf() method


/// Struct to hold option Greeks.
pub struct Greeks {
    pub delta: f64,
    pub gamma: f64,
    pub vega: f64,
}

/// Calculates Black-Scholes option Greeks for a given option.
pub fn calculate_greeks(
    spot: f64,
    strike: f64,
    time_to_expiry_days: f64,
    volatility: f64,
    is_call: bool,
) -> Greeks {
    let r = 0.06;
    let t = time_to_expiry_days / 365.0;
    let vol = volatility;
    let norm = Normal::new(0.0, 1.0).unwrap();

    let d1 = ((spot / strike).ln() + (r + 0.5 * vol.powi(2)) * t) / (vol * t.sqrt());
    let d2 = d1 - vol * t.sqrt();

    let delta = if is_call {
        norm.cdf(d1)
    } else {
        -norm.cdf(-d1)
    };

    let gamma = norm.pdf(d1) / (spot * vol * t.sqrt());
    let vega = spot * norm.pdf(d1) * t.sqrt() / 100.0;

    Greeks {
        delta,
        gamma,
        vega,
    }
}
