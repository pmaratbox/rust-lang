# 0667 — Big integer power

This computes 2 raised to the 100th power using `num_bigint::BigInt`, an arbitrary-precision integer. Native integer types overflow long before 2^100, so the `.pow(100u32)` operation runs on the big-integer type to produce the exact 31-digit value, which `Display` prints in full.

## Run

    cargo run
