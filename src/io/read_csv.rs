use crate::config::IoConfig;
use crate::error::{Result, RuptureError};
use crate::types::Bar;
use std::path::Path;

/// Read OHLCV bars from a CSV file. Column mapping is taken from the IO config.
/// Rows with missing or unparseable numeric fields are either dropped or cause
/// an error, depending on the drop_invalid_rows setting.
pub fn read_bars_csv(path: &Path, io_cfg: &IoConfig) -> Result<Vec<Bar>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_path(path)
        .map_err(RuptureError::CsvParse)?;

    let headers = reader.headers().map_err(RuptureError::CsvParse)?.clone();

    let ts_idx = find_column(&headers, &io_cfg.timestamp_column)?;
    let open_idx = find_column(&headers, &io_cfg.open_column)?;
    let high_idx = find_column(&headers, &io_cfg.high_column)?;
    let low_idx = find_column(&headers, &io_cfg.low_column)?;
    let close_idx = find_column(&headers, &io_cfg.close_column)?;
    let vol_idx = find_column(&headers, &io_cfg.volume_column)?;

    let mut bars = Vec::new();

    for (line_num, record) in reader.records().enumerate() {
        let record = record.map_err(RuptureError::CsvParse)?;
        let line = line_num + 2; // 1-indexed, plus header row

        let ts = record.get(ts_idx).unwrap_or("").to_string();

        let open = parse_f64(record.get(open_idx), line, "open");
        let high = parse_f64(record.get(high_idx), line, "high");
        let low = parse_f64(record.get(low_idx), line, "low");
        let close = parse_f64(record.get(close_idx), line, "close");
        let volume = parse_f64(record.get(vol_idx), line, "volume");

        match (open, high, low, close, volume) {
            (Ok(o), Ok(h), Ok(l), Ok(c), Ok(v)) => {
                bars.push(Bar {
                    ts,
                    open: o,
                    high: h,
                    low: l,
                    close: c,
                    volume: v,
                });
            }
            _ if io_cfg.drop_invalid_rows => continue,
            (Err(e), _, _, _, _) => return Err(e),
            (_, Err(e), _, _, _) => return Err(e),
            (_, _, Err(e), _, _) => return Err(e),
            (_, _, _, Err(e), _) => return Err(e),
            (_, _, _, _, Err(e)) => return Err(e),
        }
    }

    Ok(bars)
}

fn find_column(headers: &csv::StringRecord, name: &str) -> Result<usize> {
    headers
        .iter()
        .position(|h| h.trim().eq_ignore_ascii_case(name))
        .ok_or_else(|| {
            RuptureError::Config(format!("Column '{}' not found in CSV headers", name))
        })
}

fn parse_f64(value: Option<&str>, line: usize, field: &str) -> Result<f64> {
    let s = value.unwrap_or("").trim();
    s.parse::<f64>().map_err(|_| RuptureError::Parse {
        line,
        message: format!("cannot parse '{}' as f64 in field '{}'", s, field),
    })
}
