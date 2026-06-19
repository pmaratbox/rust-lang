# 0670 — Exact decimal multiplication

Multiply `1.1 * 1.1` using `rust_decimal::Decimal`, Rust's exact base-10 decimal
type. Unlike binary floating point (where `1.1 * 1.1` is not exactly `1.21`), the
`*` operator on `Decimal` computes the exact product, and `Display` prints `1.21`.

## Run

    cargo run
