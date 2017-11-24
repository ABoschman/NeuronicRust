use std::f32::consts::E;

/// # Examples
///
/// ```
/// use core::activation::*;
/// let large_positive = 1000.0;
/// let large_negative = -1000.0;
/// assert!(sigmoid()(large_positive) > 0.9);
/// assert!(sigmoid()(large_positive) <= 1.0);
/// assert!(sigmoid()(large_negative) >= 0.0);
/// assert!(sigmoid()(large_negative) < 0.1);
/// ```
pub fn sigmoid() -> Box<Fn(f32) -> f32> {
    Box::new(|x| 1.0 / (1.0 + E.powf(-x)))
}

pub fn mock() -> Box<Fn(f32) -> f32> {
    Box::new(|x| x)
}
