# 0408 — Take Operator

Implement take(n) over an unbounded source of the natural numbers, emitting the first 3 then completing (and unsubscribing the source). Rust shares the subscription's `active` flag via `Rc<Cell<bool>>` so the observer can halt the otherwise-infinite producer loop.

## Run

    cargo run
