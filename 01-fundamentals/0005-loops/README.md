# 0005 — Loops

Print 1..5 with a for-loop over an inclusive range. Rust's `1..=5` is the
**inclusive** range; `1..5` is half-open (stops at 4). `for` only works on
iterators — there is no C-style three-part form. Counted descending loops
use `(1..=5).rev()`.

## Run

    cargo run
