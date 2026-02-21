use crate::config::{PreprocessConfig, WindowsConfig};
use crate::error::{Result, RuptureError};
use crate::types::Bar;

/// Validate that the bar data meets minimum requirements for the engine to run.
pub fn validate_bars(
    bars: &[Bar],
    preprocess: &PreprocessConfig,
    windows: &WindowsConfig,
) -> Result<()> {
    if bars.is_empty() {
        return Err(RuptureError::EmptyData(
            "No bars remain after reading CSV".into(),
        ));
    }

    let required = preprocess
        .min_rows
        .max(windows.robust_scale_n)
        .max(windows.memory_k)
        .max(windows.capacity_l);

    if bars.len() < required {
        return Err(RuptureError::Validation(format!(
            "Need at least {} rows for the configured windows, but only {} rows are present",
            required,
            bars.len()
        )));
    }

    if preprocess.require_positive_close {
        for (i, bar) in bars.iter().enumerate() {
            if bar.close <= 0.0 {
                return Err(RuptureError::Validation(format!(
                    "Non-positive close price {:.6} at row {}",
                    bar.close, i
                )));
            }
        }
    }

    if !preprocess.allow_zero_volume {
        for (i, bar) in bars.iter().enumerate() {
            if bar.volume <= 0.0 {
                return Err(RuptureError::Validation(format!(
                    "Non-positive volume {:.2} at row {}",
                    bar.volume, i
                )));
            }
        }
    }

    for bar in bars {
        if bar.close.is_nan() || bar.volume.is_nan() {
            return Err(RuptureError::Validation(
                "NaN detected in close or volume".into(),
            ));
        }
    }

    Ok(())
}
