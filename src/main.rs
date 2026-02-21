use clap::Parser;
use std::fs;
use std::path::PathBuf;

use rupture_engine::config::Config;
use rupture_engine::io::{read_bars_csv, write_config_snapshot, write_events_json, write_timeseries_csv};
use rupture_engine::model::run_engine;
use rupture_engine::preprocess::{clean_bars, validate_bars};
use rupture_engine::reporting::diagnostics::run_diagnostics;
use rupture_engine::reporting::print_summary;

#[derive(Parser, Debug)]
#[command(
    name = "rupture-engine",
    about = "Deterministic rupture detection engine for financial time series"
)]
struct Cli {
    /// Path to the input OHLCV CSV file.
    #[arg(short, long)]
    input: PathBuf,

    /// Path to the TOML configuration file.
    #[arg(short, long)]
    config: PathBuf,

    /// Directory for output files.
    #[arg(short, long, default_value = "output")]
    output_dir: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Load configuration.
    let config = Config::from_file(&cli.config)?;

    // Ensure output directory exists.
    fs::create_dir_all(&cli.output_dir)?;

    // Read and clean input data.
    let bars = read_bars_csv(&cli.input, &config.io)?;
    let bars = clean_bars(bars, config.io.sort_by_timestamp);

    // Validate.
    validate_bars(&bars, &config.preprocess, &config.windows)?;

    // Run diagnostics.
    run_diagnostics(&bars, &config);

    // Run engine.
    let (rows, events) = run_engine(&bars, &config);

    // Write outputs.
    if config.outputs.write_csv_timeseries {
        let path = cli.output_dir.join(&config.outputs.csv_timeseries_name);
        write_timeseries_csv(&rows, &path)?;
        println!("Wrote time series CSV to {}", path.display());
    }

    if config.outputs.write_json_events {
        let path = cli.output_dir.join(&config.outputs.json_events_name);
        write_events_json(&events, &path)?;
        println!("Wrote events JSON to {}", path.display());
    }

    if config.outputs.write_json_config_snapshot {
        let path = cli.output_dir.join(&config.outputs.json_config_snapshot_name);
        write_config_snapshot(&config, &path)?;
        println!("Wrote config snapshot to {}", path.display());
    }

    // Print summary.
    print_summary(&rows, &events);

    Ok(())
}
