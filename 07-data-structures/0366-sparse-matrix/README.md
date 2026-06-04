# 0366 — Sparse Matrix

Store only nonzero entries; with (1,1)=5 set, read (1,1) (5) and (0,0) (0), printing `5 0`. A `HashMap` keyed by `(row, col)` returns 0 for any missing key via `unwrap_or`.

## Run

    cargo run
