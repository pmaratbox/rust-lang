# 0302 — Kruskal MST

Compute the MST total weight of edges (0,1,1),(1,2,2),(0,2,3) with union-find, printing `3`. `sort_by_key` on the weight plus a path-compressing `find` keeps the union-find terse.

## Run

    cargo run
