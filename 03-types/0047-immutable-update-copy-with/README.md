# 0047 — Immutable Update (Copy-with)

Make a copy of the point `(1, 2)` with its `x` changed to `9`, leaving the original intact, and print `original: (1, 2)` then `updated: (9, 2)`. The *struct update syntax* `Point { x: 9, ..p1 }` takes the remaining fields from `p1`. With `Copy`, `p1` stays usable afterward; otherwise the unspecified fields would be moved out.

## Run

    cargo run
