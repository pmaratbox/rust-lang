# 0675 — Neighbors

We build the fixed weighted undirected graph `G` with petgraph's
`UnGraphMap` (a graph keyed by `&str` node labels), then ask the library for
the adjacency of node `a` via `neighbors("a")`. The returned iterator is sorted
for determinism and comma-joined, giving `b,c`.

## Run

    cargo run
