# 0098 — Memoization

Compute `fibonacci(10)` recursively with memoization (caching each result so it is computed once) and print it: `55`. The cache is threaded through as a `&mut HashMap` (Rust has no implicit global mutable state); a hit returns early.

## Run

    cargo run
