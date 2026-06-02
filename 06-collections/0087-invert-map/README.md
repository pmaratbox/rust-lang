# 0087 — Invert a Map

Invert the map `{a: 1, b: 2, c: 3}` (swap keys and values) and print the result sorted by the new key: `1:a 2:b 3:c`. Mapping each `(k, v)` to `(v, k)` and collecting into a `BTreeMap` swaps and sorts the keys in one step.

## Run

    cargo run
