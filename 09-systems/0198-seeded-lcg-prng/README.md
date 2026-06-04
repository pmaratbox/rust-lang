# 0198 — Seeded LCG PRNG

Implement a linear congruential generator next=(5*x+3) mod 16 seeded at 1 and print its first 3 outputs `8 11 10`. The recurrence is a pure `u32` fold over an iterator, with no dependence on Rust's `rand` crate.

## Run

    cargo run
