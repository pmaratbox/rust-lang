# 0641 — Put and get

Uses the `lru` crate's strict `LruCache` (capacity 3). We `put("a", 1)` to
store the value, then `get(&"a")` retrieves it (a hit, which also promotes the
key to most-recently-used). The returned `Option` holds the value, so the
program prints `1`.

## Run

    cargo run
