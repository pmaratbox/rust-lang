# 0363 — Circular Queue

In a capacity-3 circular queue enqueue 1,2,3, dequeue once, enqueue 4, then print the contents `2 3 4`. A fixed `Vec` with head/tail indices wrapping modulo capacity keeps it idiomatic.

## Run

    cargo run
