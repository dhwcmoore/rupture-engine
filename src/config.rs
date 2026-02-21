use crate::error::{Result, RuptureError};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub io: IoConfig,
    pub preprocess: PreprocessConfig,
    pub numerics: NumericsConfig,
    pub windows: WindowsConfig,
    pub features: FeaturesConfig,
    pub robust: RobustConfig,
    pub residuals: ResidualsConfig,
    pub combine: CombineConfig,
    pub memory: MemoryConfig,
    pub capacity: CapacityConfig,
    pub state_machine: StateMachineConfig,
    pub outputs: OutputsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoConfig {
    pub timestamp_column: String,
    pub open_column: String,
    pub high_column: String,
    pub low_column: String,
    pub close_column: String,
    pub volume_column: String,
    pub parse_timestamps: bool,
    pub drop_invalid_rows: bool,
    pub sort_by_timestamp: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreprocessConfig {
    pub require_positive_close: bool,
    pub allow_zero_volume: bool,
    pub min_rows: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumericsConfig {
    pub eps: f64,
    pub clip_residuals: bool,
    pub residual_clip_max: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowsConfig {
    pub robust_scale_n: usize,
    pub memory_k: usize,
    pub capacity_l: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeaturesConfig {
    pub use_log_returns: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobustConfig {
    pub mad_scale_factor: f64,
    pub use_volume_median: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResidualsConfig {
    pub theta_vol: f64,
    pub theta_liq: f64,
    pub theta_acc: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombineConfig {
    pub tau: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    pub alpha: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityConfig {
    pub q: f64,
    pub smooth: bool,
    pub lambda: f64,
    pub e_min: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateMachineConfig {
    pub rho_stressed: f64,
    pub rho_critical: f64,
    pub rho_rupture: f64,
    pub confirm_k: usize,
    pub confirm_m: usize,
    pub enable_recovery: bool,
    pub recovery_hold: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputsConfig {
    pub write_csv_timeseries: bool,
    pub csv_timeseries_name: String,
    pub write_json_events: bool,
    pub json_events_name: String,
    pub write_json_config_snapshot: bool,
    pub json_config_snapshot_name: String,
}

impl Config {
    /// Load configuration from a TOML file.
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path).map_err(RuptureError::Io)?;
        let config: Config =
            toml::from_str(&content).map_err(|e| RuptureError::Config(e.to_string()))?;
        config.validate()?;
        Ok(config)
    }

    /// Validate parameter ranges and consistency.
    pub fn validate(&self) -> Result<()> {
        if self.memory.alpha <= 0.0 || self.memory.alpha >= 1.0 {
            return Err(RuptureError::Config(
                "memory.alpha must be in the open interval (0, 1)".into(),
            ));
        }
        if self.windows.memory_k < 10 {
            return Err(RuptureError::Config(
                "windows.memory_k must be at least 10".into(),
            ));
        }
        if self.windows.capacity_l < 50 {
            return Err(RuptureError::Config(
                "windows.capacity_l must be at least 50".into(),
            ));
        }
        if !(1..=3).contains(&self.state_machine.confirm_k) {
            return Err(RuptureError::Config(
                "state_machine.confirm_k must be 1, 2, or 3".into(),
            ));
        }
        if self.state_machine.confirm_m < 1
            || self.state_machine.confirm_m > self.state_machine.confirm_k
        {
            return Err(RuptureError::Config(
                "state_machine.confirm_m must satisfy 1 <= m <= k".into(),
            ));
        }
        if self.capacity.q <= 0.0 || self.capacity.q >= 1.0 {
            return Err(RuptureError::Config(
                "capacity.q must be in the open interval (0, 1)".into(),
            ));
        }
        if self.combine.tau <= 0.0 {
            return Err(RuptureError::Config(
                "combine.tau must be positive".into(),
            ));
        }
        Ok(())
    }
}
