# 0679 — Connectivity

We build the fixed weighted undirected graph `G` as a `petgraph::graphmap::UnGraphMap`
with `&str` node keys, then use petgraph's `has_path_connecting` reachability
algorithm to determine whether any path links node `a` to node `e`. Since `G` is
connected, the boolean result is printed lowercase as `true`.

## Run

    cargo run
