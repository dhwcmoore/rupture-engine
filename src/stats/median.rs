/// Compute the median of a mutable slice. The slice is sorted in place.
/// Returns 0.0 for empty slices.
pub fn median(values: &mut [f64]) -> f64 {
    let n = values.len();
    if n == 0 {
        return 0.0;
    }
    values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    if n % 2 == 1 {
        values[n / 2]
    } else {
        (values[n / 2 - 1] + values[n / 2]) / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median_odd() {
        let mut v = vec![3.0, 1.0, 2.0];
        assert_eq!(median(&mut v), 2.0);
    }

    #[test]
    fn test_median_even() {
        let mut v = vec![4.0, 1.0, 3.0, 2.0];
        assert_eq!(median(&mut v), 2.5);
    }

    #[test]
    fn test_median_single() {
        let mut v = vec![7.0];
        assert_eq!(median(&mut v), 7.0);
    }

    #[test]
    fn test_median_empty() {
        let mut v: Vec<f64> = vec![];
        assert_eq!(median(&mut v), 0.0);
    }
}
