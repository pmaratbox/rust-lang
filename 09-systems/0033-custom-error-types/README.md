# 0033 — Custom Error Types

Define a custom error, raise it from a `check` that rejects values over `100`, catch it for the input `200`, and print `error: value too large`. Errors are values carried by `Result<T, E>`. A custom error type implements `Display` (and usually `Error`) to render its message; the caller matches with `if let Err(e)`. The `?` operator propagates errors up the stack.

## Run

    cargo run
