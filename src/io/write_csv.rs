use crate::error::Result;
use crate::types::EngineOutputRow;
use std::path::Path;

/// Write the per-bar time series output to a CSV file.
pub fn write_timeseries_csv(rows: &[EngineOutputRow], path: &Path) -> Result<()> {
    let mut writer = csv::Writer::from_path(path)?;

    writer.write_record([
        "timestamp",
        "close",
        "volume",
        "r_vol",
        "r_liq",
        "r_acc",
        "r_combined",
        "strain",
        "capacity",
        "rho",
        "state",
        "candidate",
        "confirmed",
    ])?;

    for row in rows {
        writer.write_record(&[
            row.timestamp.clone(),
            format!("{:.6}", row.close),
            format!("{:.2}", row.volume),
            format!("{:.8}", row.r_vol),
            format!("{:.8}", row.r_liq),
            format!("{:.8}", row.r_acc),
            format!("{:.8}", row.r_combined),
            format!("{:.8}", row.strain),
            format!("{:.8}", row.capacity),
            format!("{:.8}", row.rho),
            row.state.clone(),
            if row.candidate_flag { "1" } else { "0" }.to_string(),
            if row.confirmed_flag { "1" } else { "0" }.to_string(),
        ])?;
    }

    writer.flush()?;
    Ok(())
}
