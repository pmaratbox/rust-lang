# 0113 — Result / Either Type

Model success and failure with a Result type: safeDiv(10,2) prints `ok: 5` and safeDiv(1,0) prints `err: divide by zero`. Rust's built-in `Result<i32, String>` returns `Ok`/`Err`, which a `match` destructures to print each line.

## Run

    cargo run
