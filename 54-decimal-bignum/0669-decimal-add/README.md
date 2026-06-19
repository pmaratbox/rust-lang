# 0669 — Exact decimal addition

Adding `0.1 + 0.2` with binary floating point produces `0.30000000000000004`. Using `rust_decimal::Decimal`, a base-10 exact-decimal type, the two literals are parsed and added exactly, and `Display` prints the precise value `0.3`.

## Run

    cargo run
