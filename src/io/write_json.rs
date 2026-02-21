use crate::config::Config;
use crate::error::Result;
use crate::types::RuptureEvent;
use std::fs;
use std::path::Path;

/// Write the list of detected rupture events to a JSON file.
pub fn write_events_json(events: &[RuptureEvent], path: &Path) -> Result<()> {
    let json = serde_json::to_string_pretty(events)?;
    fs::write(path, json)?;
    Ok(())
}

/// Write a snapshot of the config used for this run to a JSON file.
pub fn write_config_snapshot(config: &Config, path: &Path) -> Result<()> {
    let json = serde_json::to_string_pretty(config)?;
    fs::write(path, json)?;
    Ok(())
}
