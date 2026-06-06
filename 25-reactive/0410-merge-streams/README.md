# 0410 — Merge Streams

Implement merge of two timed streams using a virtual-time scheduler, interleaving them by emission time. A `BinaryHeap<Task>` with reversed `Ord` acts as a min-heap keyed on (time, seq) for deterministic virtual-time ordering.

## Run

    cargo run
