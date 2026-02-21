/// Numerically stable log-sum-exp soft-max combiner for residual channels.
///   r = tau * log( exp(r_vol/tau) + exp(r_liq/tau) + exp(r_acc/tau) )
/// The implementation shifts by the maximum exponent to avoid overflow.
pub fn soft_max_combine(r_vol: f64, r_liq: f64, r_acc: f64, tau: f64) -> f64 {
    let vals = [r_vol / tau, r_liq / tau, r_acc / tau];
    let max_val = vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    // If all residuals are zero, the combined residual is zero.
    if max_val == f64::NEG_INFINITY || max_val.is_nan() {
        return 0.0;
    }

    let sum_exp: f64 = vals.iter().map(|&x| (x - max_val).exp()).sum();
    tau * (max_val + sum_exp.ln())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine_all_zero() {
        let r = soft_max_combine(0.0, 0.0, 0.0, 0.35);
        // log(3) * tau ~ 0.384
        let expected = 0.35 * (3.0_f64).ln();
        assert!((r - expected).abs() < 1e-10);
    }

    #[test]
    fn test_combine_dominated() {
        // With very small tau, should approach max.
        let r = soft_max_combine(10.0, 1.0, 0.5, 0.01);
        assert!((r - 10.0).abs() < 0.1);
    }
}
