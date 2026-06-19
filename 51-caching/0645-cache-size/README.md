# 0645 — Cache size

Uses the `lru` crate's strict `LruCache` (capacity 5). After `put("a", 1)` and
`put("b", 2)`, the cache holds two live entries. `len()` reports the current
number of stored entries, so the program prints `2`. Because two entries are
well under the capacity of 5, no eviction occurs.

## Run

    cargo run
