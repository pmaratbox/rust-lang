# 0148 — Parse or Default

Parse "42" to 42 and "x" (invalid) to a default 0, printing `42 0`. Rust's `str::parse` returns a `Result`, and `unwrap_or(0)` supplies the fallback on a parse error.

## Run

    cargo run
