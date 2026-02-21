use crate::types::Bar;

/// Compute log returns from a sequence of bars. The first element is set to 0.0
/// because there is no prior close for comparison.
pub fn compute_returns(bars: &[Bar], use_log: bool) -> Vec<f64> {
    let mut returns = Vec::with_capacity(bars.len());
    returns.push(0.0);
    for i in 1..bars.len() {
        let prev = bars[i - 1].close;
        let curr = bars[i].close;
        let r = if use_log {
            (curr / prev).ln()
        } else {
            (curr - prev) / prev
        };
        returns.push(if r.is_finite() { r } else { 0.0 });
    }
    returns
}
