# 0678 — Shortest path route

We build the fixed weighted undirected graph `G` as a `petgraph::graphmap::UnGraphMap`
with `&str` node keys, then run petgraph's `astar` algorithm (with a zero
heuristic, so it behaves like Dijkstra) to recover the unique weighted shortest
path from `a` to `e`. Joining the returned node sequence with `-` yields
`a-b-c-d-e` (total cost 4).

## Run

    cargo run
