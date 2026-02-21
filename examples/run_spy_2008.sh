#!/usr/bin/env bash
# Run the rupture engine on SPY daily data (user must supply the CSV).
# Usage:
#   bash examples/run_spy_2008.sh                # expects data/spy_daily.csv
#   bash examples/run_spy_2008.sh /path/to.csv   # use a custom path
set -euo pipefail

INPUT="${1:-data/spy_daily.csv}"
OUTDIR="output/spy_2008"
BIN="./target/release/rupture-engine"

if [[ ! -f "$INPUT" ]]; then
  echo "[ERROR] Missing input CSV: $INPUT" >&2
  echo "Put your SPY daily data at: data/spy_daily.csv" >&2
  echo "Or run: bash examples/run_spy_2008.sh /full/path/to/spy_daily.csv" >&2
  exit 2
fi

# Build release binary if it is missing
if [[ ! -x "$BIN" ]]; then
  echo "[INFO] Release binary not found; building..." >&2
  cargo build --release
fi

mkdir -p "$OUTDIR"

"$BIN" \
  --input "$INPUT" \
  --config configs/daily.toml \
  --output-dir "$OUTDIR/"

echo "Done."
echo "Plot with: python scripts/plot_results.py $OUTDIR/rupture_timeseries.csv"
