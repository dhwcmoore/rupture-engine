use serde::{Deserialize, Serialize};
use std::fmt;

/// Raw OHLCV bar from input data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bar {
    pub ts: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

/// Derived features computed from consecutive bars.
#[derive(Debug, Clone, Default)]
pub struct Derived {
    pub ret: f64,
    pub abs_ret: f64,
    pub accel: f64,
}

/// Robustly normalised feature values (dimensionless).
#[derive(Debug, Clone, Default)]
pub struct Normed {
    /// Normalised absolute return: |R_t| / robust_scale_returns.
    pub u: f64,
    /// Normalised volume: V_t / median_volume.
    pub v: f64,
    /// Normalised absolute acceleration: |A_t| / robust_scale_accel.
    pub a: f64,
}

/// Individual residual channel values.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Residuals {
    pub r_vol: f64,
    pub r_liq: f64,
    pub r_acc: f64,
    pub r: f64,
}

/// Strain and capacity at a single bar.
#[derive(Debug, Clone, Default, Serialize)]
pub struct StrainPoint {
    pub s: f64,
    pub e: f64,
    pub rho: f64,
}

/// Deterministic rupture state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuptureState {
    Stable,
    Stressed,
    Critical,
    CandidateRupture,
    ConfirmedRupture,
    Recovery,
}

impl fmt::Display for RuptureState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            RuptureState::Stable => "Stable",
            RuptureState::Stressed => "Stressed",
            RuptureState::Critical => "Critical",
            RuptureState::CandidateRupture => "Candidate",
            RuptureState::ConfirmedRupture => "Confirmed",
            RuptureState::Recovery => "Recovery",
        };
        write!(f, "{}", label)
    }
}

/// Full output row for a single bar in the time series.
#[derive(Debug, Clone, Serialize)]
pub struct EngineOutputRow {
    pub timestamp: String,
    pub close: f64,
    pub volume: f64,
    pub r_vol: f64,
    pub r_liq: f64,
    pub r_acc: f64,
    pub r_combined: f64,
    pub strain: f64,
    pub capacity: f64,
    pub rho: f64,
    pub state: String,
    pub candidate_flag: bool,
    pub confirmed_flag: bool,
}

/// A detected rupture event with timing information.
#[derive(Debug, Clone, Serialize)]
pub struct RuptureEvent {
    pub candidate_index: usize,
    pub candidate_timestamp: String,
    pub confirmed_index: Option<usize>,
    pub confirmed_timestamp: Option<String>,
    pub peak_rho: f64,
    pub confirmation_k: usize,
    pub confirmation_m: usize,
}
