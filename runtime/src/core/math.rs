// Math utilities for PohLang runtime (placeholder)
pub fn add(a: f64, b: f64) -> f64 {
    a + b
}
pub fn sub(a: f64, b: f64) -> f64 {
    a - b
}
pub fn mul(a: f64, b: f64) -> f64 {
    a * b
}
pub fn div(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}
