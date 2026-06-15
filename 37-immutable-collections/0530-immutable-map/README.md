# 0530 — Immutable map

The `im` crate provides a persistent `HashMap` backed by structural sharing. Calling `update(k, v)` RETURNS A NEW map with the key set; the original map is left UNCHANGED. We build `{a:1}`, set `b=2` to get a new map, then print the new map's keys (sorted, space-joined) and the original's keys to show it was not mutated.

## Run

    cargo run
