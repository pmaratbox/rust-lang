# 0666 — Big integer multiplication

Multiply two large integers `123456789 * 987654321` using `num_bigint::BigInt`,
Rust's arbitrary-precision integer type. The `*` operator on `BigInt` computes
the exact product (`121932631112635269`) with no overflow or rounding, and
`Display` prints the full value.

## Run

    cargo run
