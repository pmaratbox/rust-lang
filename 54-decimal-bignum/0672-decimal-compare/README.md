# 0672 — Exact decimal comparison

Check whether `0.1 + 0.2` equals `0.3` using `rust_decimal::Decimal`, Rust's exact
base-10 decimal type. With binary floating point this comparison is `false`, but
`Decimal` addition is exact, so the `==` comparison against `0.3` yields `true`.

## Run

    cargo run
