# 0690 — Flat map

We map each element of `[1,2,3]` to the two-element list `[x, x*10]` and flatten
the result in a single pass using the standard iterator's `.flat_map` adapter,
then comma-join the flattened sequence with itertools' `.join(",")`, producing
`1,10,2,20,3,30`.

## Run

    cargo run
