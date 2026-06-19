# 0685 — Flatten

We take the nested list `[[1,2],[3,4],[5,6]]` and collapse it one level using the
standard iterator's `.flatten()` adapter, then comma-join the resulting sequence
with itertools' `.join(",")`, producing `1,2,3,4,5,6`.

## Run

    cargo run
