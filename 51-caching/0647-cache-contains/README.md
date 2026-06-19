# 0647 — Contains key

Uses the `lru` crate's strict `LruCache` (capacity 3). After `put("a", 1)` we
ask the cache whether each key is present with `contains(&k)`, which reports
membership *without* promoting recency (unlike `get`). Key `a` is present and
key `x` was never inserted, so the two lowercase booleans printed are
`true false`.

## Run

    cargo run
