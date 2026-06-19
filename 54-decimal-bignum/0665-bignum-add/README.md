# 0665 — Big integer addition

Both operands (`12345678901234567890` and `98765432109876543210`) are larger
than `u64::MAX`, so they cannot be held in a primitive integer. We parse each
into a `num_bigint::BigInt` — an arbitrary-precision signed integer — and add
them with `&a + &b`, yielding the exact sum `111111111011111111100`.

## Run

    cargo run
