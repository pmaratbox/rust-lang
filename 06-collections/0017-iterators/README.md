# 0017 — Iterators

Take the numbers 1 through 5, keep the even ones, double each, and add them up — a filter, then a map, then a reduce — printing the final sum. Rust's `Iterator` adapters chain lazily: `.iter().filter(...).map(...)` builds a pipeline that does no work until `.sum()` drives it. The closures see references, so the patterns dereference them — `|&&n|` in `filter` (whose predicate gets `&&i32`) and `|&n|` in `map` — and `.sum()` is a consuming reduce whose type is fixed by the `i32` annotation.

## Run

    cargo run
