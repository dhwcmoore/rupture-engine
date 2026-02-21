use crate::config::Config;
use crate::types::Bar;

/// Run basic diagnostic checks on the data and configuration and print any warnings.
pub fn run_diagnostics(bars: &[Bar], config: &Config) {
    let n = bars.len();

    if n < config.windows.capacity_l * 2 {
        eprintln!(
            "[WARN] Only {} bars, which is less than 2x the capacity window ({}). \
             Capacity estimates will be based on partial data for much of the run.",
            n,
            config.windows.capacity_l
        );
    }

    if config.memory.alpha < 0.3 {
        eprintln!(
            "[WARN] memory.alpha = {:.2} is quite low, producing a very long memory tail. \
             This may make the model slow to adapt to regime changes.",
            config.memory.alpha
        );
    }

    if config.memory.alpha > 0.9 {
        eprintln!(
            "[WARN] memory.alpha = {:.2} is close to 1, producing very short memory. \
             The model may behave more like a simple moving average.",
            config.memory.alpha
        );
    }

    // Check for zero-volume bars.
    let zero_vol_count = bars.iter().filter(|b| b.volume == 0.0).count();
    if zero_vol_count > 0 {
        eprintln!(
            "[WARN] {} bars ({:.1}%) have zero volume. The liquidity residual channel \
             may produce spurious signals for these bars.",
            zero_vol_count,
            100.0 * zero_vol_count as f64 / n as f64
        );
    }

    // Check for large gaps in price.
    let mut large_gap_count = 0;
    for i in 1..bars.len() {
        let ratio = bars[i].close / bars[i - 1].close;
        if ratio > 1.2 || ratio < 0.8 {
            large_gap_count += 1;
        }
    }
    if large_gap_count > 0 {
        eprintln!(
            "[WARN] {} bar-to-bar price changes exceed 20%. Consider checking for \
             data quality issues (stock splits, corporate actions, bad data).",
            large_gap_count
        );
    }
}
