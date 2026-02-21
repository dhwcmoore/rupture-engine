use crate::stats::{mad, median, safe_div, RollingWindow};
use crate::types::{Bar, Normed};

/// Compute robustly normalised features for each bar using rolling MAD for returns
/// and acceleration, and rolling median for volume. The first `window_n` bars will
/// use partial windows (computing over whatever data is available so far).
pub fn compute_normed(
    bars: &[Bar],
    returns: &[f64],
    accel: &[f64],
    window_n: usize,
    mad_scale_factor: f64,
    eps: f64,
) -> Vec<Normed> {
    let n = bars.len();
    let mut normed = Vec::with_capacity(n);

    let mut ret_win = RollingWindow::new(window_n);
    let mut acc_win = RollingWindow::new(window_n);
    let mut vol_win = RollingWindow::new(window_n);

    for i in 0..n {
        ret_win.push(returns[i]);
        acc_win.push(accel[i]);
        vol_win.push(bars[i].volume);

        let ret_snapshot = ret_win.as_slice();
        let acc_snapshot = acc_win.as_slice();
        let vol_snapshot = vol_win.as_slice();

        let sigma_ret = mad_scale_factor * mad(&ret_snapshot) + eps;
        let sigma_acc = mad_scale_factor * mad(&acc_snapshot) + eps;
        let vol_med = {
            let mut v = vol_snapshot;
            median(&mut v) + eps
        };

        let u = safe_div(returns[i].abs(), sigma_ret, 0.0);
        let v = safe_div(bars[i].volume, vol_med, 0.0);
        let a = safe_div(accel[i].abs(), sigma_acc, 0.0);

        normed.push(Normed { u, v, a });
    }

    normed
}
