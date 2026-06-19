# 0646 — Update a value

Uses the `lru` crate's strict `LruCache` (capacity 3). We `put("a", 1)` and
then `put("a", 2)` with the same key — the crate updates the existing entry's
value in place rather than adding a second one. A `get(&"a")` (a hit, which
also promotes the key to most-recently-used) returns the latest value, so the
program prints `2`.

## Run

    cargo run
