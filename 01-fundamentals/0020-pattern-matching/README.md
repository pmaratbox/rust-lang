# 0020 — Pattern Matching

Match `n` against the literal patterns `1` and `2` with a wildcard fallback, mapping `1`, `2`, and `5` to `one`, `two`, and `many`. `match` is an expression and must be exhaustive — the compiler rejects it unless every value is covered, which the `_` wildcard guarantees. Each arm yields a value, so `word` returns the matched `&'static str` directly; patterns can also bind, destructure, and carry guards.

## Run

    cargo run
