# 0080 — Deduplicate

Remove duplicates from `1, 2, 2, 3, 1`, keeping the first occurrence of each in order, and print `1 2 3`. `HashSet::insert` returns `true` only the first time a value is seen, so `filter` keeps each value's first occurrence in order.

## Run

    cargo run
