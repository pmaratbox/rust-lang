# 0412 — Combine Latest

Implement combineLatest of two timed streams, emitting the pair of latest values whenever either source emits (once both have emitted). A `BinaryHeap` ordered by `(time, seq)` drives a virtual-time scheduler with `Rc<RefCell<_>>` holding each source's latest value.

## Run

    cargo run
