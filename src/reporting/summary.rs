use crate::types::{EngineOutputRow, RuptureEvent};

/// Print a human-readable summary of the engine run to stdout.
pub fn print_summary(rows: &[EngineOutputRow], events: &[RuptureEvent]) {
    let total_bars = rows.len();
    let candidates: usize = events.len();
    let confirmed: usize = events.iter().filter(|e| e.confirmed_index.is_some()).count();
    let unconfirmed = candidates - confirmed;

    println!("=== Rupture Engine Summary ===");
    println!("Total bars processed: {}", total_bars);
    println!("Candidate ruptures:   {}", candidates);
    println!("Confirmed ruptures:   {}", confirmed);
    println!("Unconfirmed candidates: {}", unconfirmed);

    if confirmed > 0 {
        let lags: Vec<usize> = events
            .iter()
            .filter_map(|e| {
                e.confirmed_index
                    .map(|ci| ci.saturating_sub(e.candidate_index))
            })
            .collect();
        let avg_lag = lags.iter().sum::<usize>() as f64 / lags.len() as f64;
        println!("Average confirmation lag: {:.1} bars", avg_lag);

        let peak_rhos: Vec<f64> = events
            .iter()
            .filter(|e| e.confirmed_index.is_some())
            .map(|e| e.peak_rho)
            .collect();
        let max_peak = peak_rhos.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        println!("Maximum peak rho (confirmed): {:.4}", max_peak);
    }

    // State distribution.
    let mut state_counts = std::collections::HashMap::new();
    for row in rows {
        *state_counts.entry(row.state.clone()).or_insert(0usize) += 1;
    }
    println!("\nState distribution:");
    for (state, count) in &state_counts {
        let pct = 100.0 * *count as f64 / total_bars as f64;
        println!("  {:<12} {:>6} bars ({:.1}%)", state, count, pct);
    }
    println!("==============================");
}
