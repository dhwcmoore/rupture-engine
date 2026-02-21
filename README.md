# Rupture Engine

A deterministic Rust CLI for detecting stress build-up and regime transitions in financial time series.

The engine ingests OHLCV bar data and produces:

- A per-bar stress time series (CSV)
- A structured rupture event log (JSON)
- A configuration snapshot for full reproducibility

This project demonstrates production-style modelling in Rust: robust statistics, adaptive thresholds, deterministic state transitions, and test coverage.

---

## Example output (SPY daily)

![Rupture Engine output](docs/spy_daily_example.png)

---

## What this project demonstrates

- Rolling robust statistics (median, MAD, quantiles)
- Multi-channel feature engineering
- Weighted rolling convolution (long-memory accumulation)
- Adaptive thresholding via rolling quantiles
- Deterministic finite-state machine with m-of-k confirmation logic
- CLI argument parsing and config-driven execution
- Structured output schemas (CSV + JSON)
- Unit and integration tests in Rust

This is a reproducible command-line data pipeline.

---

## System architecture

    OHLCV CSV
      |
      v
    Feature layer (returns, acceleration, volume)
      |
      v
    Residual layer (r_vol, r_liq, r_acc)
      |
      v
    Memory kernel (weighted accumulation)
      |
      v
    Adaptive capacity (rolling quantile)
      |
      v
    State machine (Stable → … → Confirmed/Recovery)
      |
      v
    CSV time series + JSON event log

---

## Pipeline overview

### 1) Feature extraction

From OHLCV bars:

- Log returns
- Return acceleration
- Volume

Each feature is robustly normalised using rolling median and MAD.

### 2) Residual channels

Excess activity beyond configurable thresholds:

    r_vol = max(0, u - θ_vol)
    r_liq = max(0, u / (v + ε) - θ_liq)
    r_acc = max(0, a - θ_acc)

Channels are combined using a numerically stable soft-max.

### 3) Strain accumulation

Residuals are accumulated using a weighted rolling window, producing a strain signal that captures persistent stress.

### 4) Adaptive capacity

Capacity is estimated as a rolling quantile of historical strain with optional smoothing. This allows thresholds to adapt to changing volatility regimes.

### 5) Deterministic state machine

    rho = strain / capacity

drives transitions through:

- Stable
- Stressed
- Critical
- Candidate
- Confirmed
- Recovery

Confirmed ruptures require m-of-k confirmation logic.

---

## Example: SPY daily (2005–2026)

Applied to 5,000+ daily SPY bars:

- Stable: ~74%
- Stressed: ~18%
- Critical: ~2%
- Confirmed rupture episodes: 13

Major stress regimes identified:

- 2007–08 global selloff
- 2011 US downgrade / Euro crisis
- 2015 China devaluation shock
- 2016 Brexit
- 2018 volatility spike
- 2020 COVID acceleration

---

## Quick start

Build:

    cargo build --release

Run:

    ./target/release/rupture-engine \
      --input data/spy_daily.csv \
      --config configs/default.toml \
      --output-dir output/

Or:

    bash examples/run_spy_2008.sh /path/to/spy_daily.csv

Plot:

    python scripts/plot_results.py output/spy_2008/rupture_timeseries.csv

---

## Configuration

All parameters are defined in a single TOML file, including:

- Residual thresholds
- Memory window and exponent
- Capacity quantile
- State transition levels
- Confirmation window (k, m)

Each run writes config_used.json for reproducibility.

---

## Outputs

### rupture_timeseries.csv

Per-bar fields include:

- timestamp
- residual channels
- strain
- capacity
- rho
- state
- candidate flag
- confirmed flag

### rupture_events.json

Structured event records:

- candidate timestamp
- confirmation timestamp
- peak rho
- confirmation parameters

---

## Testing

    cargo test

Includes:

- Rolling statistic tests
- Memory kernel tests
- State machine transition tests
- CLI smoke test

---

## Limitations

- Single instrument
- Deterministic and parameter-sensitive
- Designed for daily or moderate-frequency data

---

## License

MIT