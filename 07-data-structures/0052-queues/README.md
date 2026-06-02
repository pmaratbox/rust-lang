# 0052 — Queues

Enqueue `1`, `2`, and `3` into a queue, then dequeue them all and print them in first-in-first-out order: `1 2 3`. `VecDeque` is the standard double-ended queue: `push_back` enqueues and `pop_front` returns `Option<T>`, so `while let Some(n)` drains it.

## Run

    cargo run
