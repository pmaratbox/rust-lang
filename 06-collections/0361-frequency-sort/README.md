# 0361 — Frequency Sort

Sort [1,1,2,3,3,3] by descending frequency (ties keep first-seen order), printing `3 3 3 1 1 2`. Rust counts in a `HashMap` and applies a stable `sort_by` on the descending count.

## Run

    cargo run
