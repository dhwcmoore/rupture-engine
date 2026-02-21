#!/usr/bin/env python3
"""
Plot rupture engine output time series.

Usage:
    python scripts/plot_results.py output/rupture_timeseries.csv

Requires: matplotlib, pandas
    pip install matplotlib pandas
"""

import sys
import os
import pandas as pd
import matplotlib.pyplot as plt


def main():
    if len(sys.argv) < 2 or not os.path.exists(sys.argv[1]):
        print(f"[ERROR] Missing input file: {sys.argv[1] if len(sys.argv) > 1 else '<none>'}")
        print("Run the engine first, e.g.:")
        print("  bash examples/run_spy_2008.sh")
        sys.exit(2)

    df = pd.read_csv(sys.argv[1])

    # Prefer timestamp on x-axis if it parses, otherwise fall back to index.
    x = df.index
    x_label = "Bar index"
    if "timestamp" in df.columns:
        ts = pd.to_datetime(df["timestamp"], errors="coerce")
        if ts.notna().any():
            x = ts
            x_label = "Date"

    fig, axes = plt.subplots(4, 1, figsize=(14, 10), sharex=True)

    # Panel 1: Close price
    ax = axes[0]
    ax.plot(x, df["close"], color="black", linewidth=0.8)
    ax.set_ylabel("Close")
    ax.set_title("Rupture Engine Output")

    # Panel 2: Combined residual
    ax = axes[1]
    ax.plot(x, df["r_combined"], color="steelblue", linewidth=0.6)
    ax.set_ylabel("Combined r")

    # Panel 3: Strain and capacity
    ax = axes[2]
    ax.plot(x, df["strain"], color="crimson", linewidth=0.8, label="Strain S")
    ax.plot(x, df["capacity"], color="grey", linewidth=0.8, linestyle="--", label="Capacity E")
    ax.set_ylabel("S / E")
    ax.legend(loc="upper left", fontsize=8)

    # Panel 4: Rho
    ax = axes[3]
    ax.plot(x, df["rho"], color="darkred", linewidth=0.8)
    ax.axhline(y=1.0, color="black", linestyle=":", linewidth=0.5)
    ax.axhline(y=0.85, color="grey", linestyle=":", linewidth=0.5)
    ax.axhline(y=0.60, color="lightgrey", linestyle=":", linewidth=0.5)
    ax.set_ylabel("rho")
    ax.set_xlabel(x_label)

    # Mark confirmed only (no candidates)
    if "confirmed" in df.columns:
        conf_mask = (df["confirmed"] == 1)
        if conf_mask.any():
            conf_x = x[conf_mask] if not isinstance(x, pd.RangeIndex) else df.index[conf_mask]
            for xi in conf_x:
                axes[0].axvline(x=xi, color="darkred", linewidth=0.8, alpha=0.6)
                axes[3].axvline(x=xi, color="darkred", linewidth=0.8, alpha=0.6)
    plt.tight_layout()
    out_path = sys.argv[1].replace(".csv", ".png")
    plt.savefig(out_path, dpi=150)
    print(f"Saved plot to {out_path}")
    plt.close()


if __name__ == "__main__":
    main()
