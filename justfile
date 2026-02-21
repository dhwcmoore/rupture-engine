default:
    @just --list

fmt:
    cargo fmt

lint:
    cargo clippy -- -D warnings

test:
    cargo test

build:
    cargo build --release

run *ARGS:
    cargo run --release -- {{ARGS}}

run-example:
    cargo run --release -- \
        --input data/fixtures/tiny_ohlcv.csv \
        --config configs/default.toml \
        --output-dir output/

clean:
    cargo clean
    rm -rf output/
