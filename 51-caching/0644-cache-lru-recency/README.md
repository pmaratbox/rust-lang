# 0644 — Recency promotion

Uses the `lru` crate's strict `LruCache` (capacity 3). We put `a=1, b=2, c=3`,
then `get(&"a")` — a hit that promotes `a` to most-recently-used, leaving `b`
as the least-recently-used key. Inserting `d=4` overflows capacity and evicts
`b`. So `get(&"a")` still returns `1`, while `get(&"b")` is a miss, printing
`1 miss`.

## Run

    cargo run
