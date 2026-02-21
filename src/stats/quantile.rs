/// Compute the q-th quantile of a slice using linear interpolation.
/// q should be in [0, 1]. The slice is not modified; a sorted copy is used internally.
/// Returns 0.0 for empty slices.
pub fn quantile(values: &[f64], q: f64) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let mut sorted: Vec<f64> = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let n = sorted.len();
    if n == 1 {
        return sorted[0];
    }
    let pos = q * (n - 1) as f64;
    let lo = pos.floor() as usize;
    let hi = pos.ceil() as usize;
    if lo == hi {
        sorted[lo]
    } else {
        let frac = pos - lo as f64;
        sorted[lo] * (1.0 - frac) + sorted[hi] * frac
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantile_median() {
        let v = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert!((quantile(&v, 0.5) - 3.0).abs() < 1e-12);
    }

    #[test]
    fn test_quantile_extremes() {
        let v = vec![10.0, 20.0, 30.0];
        assert!((quantile(&v, 0.0) - 10.0).abs() < 1e-12);
        assert!((quantile(&v, 1.0) - 30.0).abs() < 1e-12);
    }

    #[test]
    fn test_quantile_interpolation() {
        let v = vec![0.0, 10.0];
        assert!((quantile(&v, 0.3) - 3.0).abs() < 1e-12);
    }
}
