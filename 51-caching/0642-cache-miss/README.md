# 0642 — Cache miss

Uses the `lru` crate's strict `LruCache` (capacity 3). Looking up key `x` in an
empty cache, `get(&"x")` returns `None`, so the lookup is a cache miss and we
print `miss`. The result comes straight from the cache's lookup behavior, not a
hardcoded string.

## Run

    cargo run
