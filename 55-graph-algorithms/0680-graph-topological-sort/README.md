# 0680 — Topological sort

We build the fixed DAG as a `petgraph::graphmap::DiGraphMap` with `&str` node
keys, then run petgraph's `toposort` algorithm to compute a topological
ordering of the vertices. The DAG (`a->b`, `b->c`, `a->c`, `c->d`, `d->e`) has
the unique order `a,b,c,d,e`, which is printed comma-joined.

## Run

    cargo run
