/// Compute acceleration as the first difference of returns: A_t = R_t - R_{t-1}.
/// The first element is set to 0.0.
pub fn compute_acceleration(returns: &[f64]) -> Vec<f64> {
    let mut accel = Vec::with_capacity(returns.len());
    accel.push(0.0);
    for i in 1..returns.len() {
        accel.push(returns[i] - returns[i - 1]);
    }
    accel
}
