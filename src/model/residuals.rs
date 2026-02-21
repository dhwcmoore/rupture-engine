use crate::config::{NumericsConfig, ResidualsConfig};
use crate::stats::robust::clip;
use crate::types::{Normed, Residuals};

/// Compute the three residual channels from normalised features.
///   r_vol = max(0, u - theta_vol)
///   r_liq = max(0, u / (v + eps) - theta_liq)
///   r_acc = max(0, a - theta_acc)
/// Residuals are optionally clipped to prevent outlier domination.
pub fn compute_residuals(
    normed: &Normed,
    residuals_cfg: &ResidualsConfig,
    numerics_cfg: &NumericsConfig,
) -> Residuals {
    let mut r_vol = (normed.u - residuals_cfg.theta_vol).max(0.0);
    let mut r_liq = (normed.u / (normed.v + numerics_cfg.eps) - residuals_cfg.theta_liq).max(0.0);
    let mut r_acc = (normed.a - residuals_cfg.theta_acc).max(0.0);

    if numerics_cfg.clip_residuals {
        r_vol = clip(r_vol, numerics_cfg.residual_clip_max);
        r_liq = clip(r_liq, numerics_cfg.residual_clip_max);
        r_acc = clip(r_acc, numerics_cfg.residual_clip_max);
    }

    Residuals {
        r_vol,
        r_liq,
        r_acc,
        r: 0.0, // combined is set later by the combiner
    }
}
