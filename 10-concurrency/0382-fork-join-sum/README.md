# 0382 — Fork-Join Sum

Recursively fork the sum of [1..8] into halves and join the partial sums, printing `36`. Rust's `thread::scope` lets each recursion borrow the slice and join its two child threads.

## Run

    cargo run
