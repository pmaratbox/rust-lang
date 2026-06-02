# 0034 — Default & Named Arguments

Give a `greet` function a default greeting, then call it once without the greeting and once overriding it, printing `Hello, Ada` and `Hi, Ada`. Rust has no default or named arguments. Common substitutes are an `Option` parameter with `unwrap_or` for the default (used here), the builder pattern, or a struct implementing `Default`.

## Run

    cargo run
