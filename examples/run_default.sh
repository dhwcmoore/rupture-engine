#!/usr/bin/env bash
set -euo pipefail

./target/release/rupture-engine \
  --input data/fixtures/tiny_ohlcv_60.csv \
  --config configs/tiny.toml \
  --output-dir output/tiny_test/
