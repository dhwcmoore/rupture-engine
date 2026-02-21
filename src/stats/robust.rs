/// Safe division that returns a / (b + eps) to avoid division by zero.
pub fn safe_div(a: f64, b: f64, eps: f64) -> f64 {
    a / (b + eps)
}

/// Winsorise (clip) a value to the range [0, max].
pub fn clip(value: f64, max: f64) -> f64 {
    value.max(0.0).min(max)
}
