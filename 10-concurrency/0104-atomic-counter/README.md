# 0104 — Atomic Counter

Increment a shared atomic counter from multiple threads 1000 times total without a lock, printing `1000`. An `AtomicUsize` shared via `Arc` uses `fetch_add` for lock-free increments.

## Run

    cargo run
