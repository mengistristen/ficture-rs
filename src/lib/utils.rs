//! A module containing helpful functions that are used throughout
//! this crate.

/// Normalizes a value within the range min-max to a value
/// from 0-1.
pub fn normalize(value: f64, min: f64, max: f64) -> f64 {
    (value - min) / (max - min)
}
