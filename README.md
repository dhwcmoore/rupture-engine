# Rupture Engine

A deterministic Rust CLI for modelling stress accumulation and regime transitions in time-indexed financial data.

The engine treats regime detection as a state transition problem over a fully specified data pipeline. All transformations are explicit, parameterised, and reproducible.

Given OHLCV bar data, the engine produces:

* A per-bar stress time series (CSV)
* A structured rupture event log (JSON)
* A configuration snapshot for full reproducibility

No stochastic components are used. All outputs are deterministic functions of input data and configuration.

---

## Example output (SPY daily)

![Rupture Engine output](docs/spy_daily_example.png)

---

## Design principles

* Deterministic transformations only
* Explicit intermediate representations
* Config-driven parameterisation
* Invariant-preserving state transitions
* Structured, machine-readable outputs

The system is organised as a compositional pipeline whose stages are independently testable.

---

## System architecture

```id="xw8k27"
OHLCV CSV
   ↓
Feature extraction
   ↓
Residual channels
   ↓
Memory kernel (strain)
   ↓
Adaptive capacity
   ↓
Finite-state machine
   ↓
CSV time series + JSON event log
```

Each stage is a pure transformation over time-indexed data.

---

## 1. Feature layer

From OHLCV bars:

* Log returns
* Return acceleration
* Volume

Each feature is robustly normalised using rolling median and MAD.

All rolling statistics are explicitly implemented and covered by tests.

---

## 2. Residual channels

Thresholded excess activity is defined per channel:

```id="3sfl29"
r_vol = max(0, u - θ_vol)
r_liq = max(0, u / (v + ε) - θ_liq)
r_acc = max(0, a - θ_acc)
```

Residual channels are combined using a numerically stable soft-max.

All thresholds are defined in a TOML configuration file and persisted per run.

---

## 3. Strain accumulation (memory kernel)

Residuals are accumulated via a weighted rolling convolution.

This produces a strain signal representing persistent stress rather than isolated deviations.

The kernel window and weighting exponent are configurable and deterministic.

---

## 4. Adaptive capacity

Capacity is defined as a rolling quantile of historical strain with optional smoothing.

```id="k8s2mz"
rho = strain / capacity
```

The ratio `rho` is the sole driver of regime transitions.

Capacity adapts to regime scale while remaining an explicit, observable signal.

---

## 5. Finite-state machine

Regime classification is implemented as an explicit finite-state machine:

* Stable
* Stressed
* Critical
* Candidate
* Confirmed
* Recovery

Transitions are deterministic functions of `rho` and confirmation logic.

Confirmed ruptures require m-of-k confirmation over a configurable window.

All state transitions are encoded explicitly and tested.

---

## Example: SPY daily (2005–2026)

Applied to approximately 5,000 daily bars:

* Stable: ~74%
* Stressed: ~18%
* Critical: ~2%
* Confirmed rupture episodes: 13

These correspond to major volatility regimes.
Results are parameter-dependent but reproducible.

---

## Build

```id="3qk2lg"
cargo build --release
```

---

## Run

```id="t91zab"
./target/release/rupture-engine \
  --input data/spy_daily.csv \
  --config configs/default.toml \
  --output-dir output/
```

Each run writes:

* `rupture_timeseries.csv`
* `rupture_events.json`
* `config_used.json`

The configuration snapshot ensures that outputs are reproducible.

---

## Outputs

### rupture_timeseries.csv

Per-bar structured fields:

* timestamp
* residual channels
* strain
* capacity
* rho
* state
* candidate flag
* confirmed flag

### rupture_events.json

Structured event records:

* candidate timestamp
* confirmation timestamp
* peak rho
* confirmation parameters

All outputs are machine-readable and schema-consistent.

---

## Testing

```id="qun7m2"
cargo test
```

Test coverage includes:

* Rolling statistic correctness
* Memory kernel behaviour
* State transition invariants
* CLI integration

---

## Limitations

* Single-instrument design
* Parameter-sensitive thresholds
* Deterministic model (not probabilistic)
* No execution or cost modelling
* Not a trading system

This repository demonstrates a deterministic regime detection architecture, not an investment strategy.

---

## License

MIT
