# 0298 — Bellman-Ford

On the digraph with a negative edge 0->1(1),1->2(-2),0->2(4), print the shortest distances from node 0 `0 1 -1`. A flat slice of `(u, v, w)` tuples relaxed in a loop keeps the algorithm compact in Rust.

## Run

    cargo run
