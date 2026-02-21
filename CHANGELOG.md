# Changelog

## 0.1.0 (unreleased)

Initial release of the deterministic rupture engine. Core pipeline includes three-channel robust residuals (volatility, liquidity, acceleration), power-law long-memory strain accumulation, rolling-quantile adaptive capacity with optional smoothing, and a deterministic state machine with configurable m-of-k confirmation. Outputs include per-bar CSV time series and JSON event listings. Tests cover rolling statistics, memory kernel, state machine transitions, and CLI smoke testing.
