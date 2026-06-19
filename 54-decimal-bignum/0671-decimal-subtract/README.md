# 0671 — Exact decimal subtraction

Subtract `1.0 - 0.1` using `rust_decimal::Decimal`, Rust's exact base-10 decimal
type. Unlike binary floating point (where `1.0 - 0.1` is not exactly `0.9`), the
`-` operator on `Decimal` computes the exact difference, and `Display` prints `0.9`.

## Run

    cargo run
