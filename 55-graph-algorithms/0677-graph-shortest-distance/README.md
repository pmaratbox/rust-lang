# 0677 — Shortest distance

We build the fixed weighted undirected graph `G` as a `petgraph::graphmap::UnGraphMap`
with `&str` node keys, then run petgraph's `dijkstra` algorithm to compute the
weighted shortest-path distance from `a` to `e`. The unique shortest path
`a-b-c-d-e` has total cost `4`, which is printed.

## Run

    cargo run
