# 0164 — Merge Maps

Merge {a:1,b:2} and {b:3,c:4} with the right map winning on conflicts, printing `a:1 b:3 c:4`. A `BTreeMap` keeps keys sorted and right-biased inserts overwrite on conflict.

## Run

    cargo run
