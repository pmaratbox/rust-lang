# 0068 — GCD (Euclid)

Compute the greatest common divisor of `48` and `36` with Euclid's algorithm (repeatedly replace the pair with `(b, a % b)` until the remainder is zero) and print it: `12`. The `mut` parameters are rebound each step via a temporary `t`; when `b` reaches zero, `a` holds the GCD.

## Run

    cargo run
