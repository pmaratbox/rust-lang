# 0643 — LRU eviction

Uses the `lru` crate's `LruCache` (a strict least-recently-used cache) with
capacity 3. Keys are inserted in order `a=1, b=2, c=3, d=4` with no lookups in
between, so when `d` is put the least-recently-used key `a` is evicted. A
`get(&"a")` then returns `None` (printed as `miss`) while `get(&"d")` returns
`4`, yielding `miss 4`.

## Run

    cargo run
