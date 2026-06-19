# 0648 — Capacity bound

Uses the `lru` crate's `LruCache` (a strict LRU cache). With capacity 3 we
put four items (`a`, `b`, `c`, `d`); the least-recently-used entry is evicted
when the fourth is inserted, so `len()` reports the size capped at the
capacity rather than the four items inserted.

## Run

    cargo run
