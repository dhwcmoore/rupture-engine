use crate::types::Bar;

/// Clean bars by deduplicating timestamps and optionally sorting. Rows with
/// impossible values (negative close) are removed.
pub fn clean_bars(bars: Vec<Bar>, sort: bool) -> Vec<Bar> {
    let mut cleaned = bars;

    if sort {
        cleaned.sort_by(|a, b| a.ts.cmp(&b.ts));
    }

    // Remove duplicate timestamps, keeping the first occurrence.
    cleaned.dedup_by(|a, b| a.ts == b.ts);

    // Remove rows with impossible values.
    cleaned.retain(|bar| bar.close > 0.0 && !bar.close.is_nan() && !bar.volume.is_nan());

    cleaned
}
