# 0013 — Optional

Hold one value that is present (`42`) and one that is absent, then print each
with a fallback of `-1` when absent. Rust models optionality with the
`Option<T>` enum — `Some(value)` or `None` — and `.unwrap_or(fallback)` reads
the value or substitutes the fallback. There is no null.

## Run

    cargo run
