# 0375 — Retry With Attempts

Retry an operation that fails on attempts 1 and 2 and succeeds on attempt 3, printing `ok after 3`. Rust loops over an inclusive `1..=N` range and `break`s on the first success.

## Run

    cargo run
