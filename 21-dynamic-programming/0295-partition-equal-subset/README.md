# 0295 — Equal Partition

Decide whether [1,5,11,5] can split into two equal-sum subsets, printing `yes`. Rust reduces the half-sum target with `iter().sum()` then runs a boolean subset-sum DP.

## Run

    cargo run
