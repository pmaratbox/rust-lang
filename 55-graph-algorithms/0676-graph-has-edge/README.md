# 0676 — Edge existence

We build the fixed weighted undirected graph `G` as a `petgraph::graphmap::UnGraphMap`
with `&str` node keys, then use the library's `contains_edge` lookup to test for
the presence of two edges: `b-c` (present) and `a-e` (absent). The booleans are
printed lowercase and space-joined, yielding `true false`.

## Run

    cargo run
