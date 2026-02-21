use crate::stats::{quantile, RollingWindow};

/// Adaptive capacity estimator using a rolling quantile of the strain history.
/// Optionally applies exponential smoothing for stability.
pub struct CapacityEstimator {
    window: RollingWindow,
    q: f64,
    smooth: bool,
    lambda: f64,
    e_min: f64,
    prev_e: Option<f64>,
}

impl CapacityEstimator {
    pub fn new(window_l: usize, q: f64, smooth: bool, lambda: f64, e_min: f64) -> Self {
        Self {
            window: RollingWindow::new(window_l),
            q,
            smooth,
            lambda,
            e_min,
            prev_e: None,
        }
    }

    /// Update the capacity estimate with a new strain value and return the current capacity.
    pub fn update(&mut self, strain: f64) -> f64 {
        self.window.push(strain);
        let snapshot = self.window.as_slice();
        let e_raw = quantile(&snapshot, self.q).max(self.e_min);

        let e = if self.smooth {
            match self.prev_e {
                Some(prev) => (1.0 - self.lambda) * prev + self.lambda * e_raw,
                None => e_raw,
            }
        } else {
            e_raw
        };

        let e = e.max(self.e_min);
        self.prev_e = Some(e);
        e
    }
}
