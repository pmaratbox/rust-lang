# 0422 — Throttle (Virtual Time)

Implement throttle(window) (leading edge) on a virtual-time scheduler: emit a value, then suppress further values for `window` ticks. A `BinaryHeap<Entry>` with reversed `Ord` acts as the min-heap priority queue, breaking ties on the insertion `seq`.

## Run

    cargo run
