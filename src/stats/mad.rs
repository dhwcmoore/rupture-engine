use crate::stats::median::median;

/// Compute the median absolute deviation (MAD) of a slice.
/// MAD = median(|x_i - median(x)|).
/// Returns 0.0 for empty slices.
pub fn mad(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let mut buf = values.to_vec();
    let med = median(&mut buf);
    let mut deviations: Vec<f64> = values.iter().map(|x| (x - med).abs()).collect();
    median(&mut deviations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mad_symmetric() {
        // For values [1,2,3,4,5], median = 3, deviations = [2,1,0,1,2], MAD = 1.
        let v = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert!((mad(&v) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_mad_constant() {
        let v = vec![5.0, 5.0, 5.0, 5.0];
        assert_eq!(mad(&v), 0.0);
    }
}
