# 0321 — Lazy Take

Build a lazy sequence of the natural numbers and take the first five, printing `1 2 3 4 5`. Rust's `(1..)` range iterator is lazy, so `.take(5)` only realizes the needed elements.

## Run

    cargo run
