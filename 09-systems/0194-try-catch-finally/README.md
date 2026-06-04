# 0194 — Try / Catch / Finally

Throw and catch an error, printing `caught`, and always run a finally block printing `cleanup`, on two lines. Rust has no exceptions for recoverable errors, so match on a `Result` and run the cleanup unconditionally afterward.

## Run

    cargo run
