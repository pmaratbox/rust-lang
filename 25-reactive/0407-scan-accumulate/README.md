# 0407 — Scan (Running Fold)

Implement a scan operator that emits the running accumulation; produce the running sums of 1, 2, 3, 4. The accumulator state is captured by a `move` closure boxed as `Box<dyn Fn>`, so each subscription threads its own mutable running total.

## Run

    cargo run
