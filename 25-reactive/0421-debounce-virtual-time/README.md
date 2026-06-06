# 0421 — Debounce (Virtual Time)

Implement debounce(window) on a virtual-time scheduler: emit a value only after a quiet gap of `window` ticks with no newer value. In Rust we model cancel tokens with `Rc<RefCell<bool>>` and order the `BinaryHeap` by reversed `(time, seq)`.

## Run

    cargo run
