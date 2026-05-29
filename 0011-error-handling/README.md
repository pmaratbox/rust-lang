# 0011 — Error Handling

Write a `divide(a, b)` that reports a zero divisor, then call it on `10 / 2`
(prints the result) and `10 / 0` (prints an error). Rust returns `Result<T, E>`
— `Ok(value)` or `Err(message)` — and the caller handles both arms with
`match`. There are no exceptions for recoverable errors.

## Run

    cargo run
