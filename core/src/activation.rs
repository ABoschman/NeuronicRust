use std::f32::consts::E;

/// # Examples
///
/// ```
/// use core::activation::*;
/// let large_positive = 1000.0;
/// let large_negative = -1000.0;
/// assert!(make_sigmoid()(large_positive) > 0.9);
/// assert!(make_sigmoid()(large_positive) <= 1.0);
/// assert!(make_sigmoid()(large_negative) >= 0.0);
/// assert!(make_sigmoid()(large_negative) < 0.1);
/// ```
pub fn make_sigmoid() -> Box<Fn(f32) -> f32> {
    Box::new(|x| 1.0 / (1.0 + E.powf(-x)))
}

pub fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + E.powf(-x))
}

pub fn mock() -> Box<Fn(f32) -> f32> {
    Box::new(|x| x)
}
