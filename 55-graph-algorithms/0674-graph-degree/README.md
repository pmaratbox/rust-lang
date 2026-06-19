# 0674 — Node degree

We build the fixed weighted undirected graph `G` with the `petgraph` crate's
`UnGraphMap`, then ask the library for the degree of node `b` by counting its
incident neighbors (`g.neighbors("b").count()`). Since `b` connects to `a`, `c`,
and `d`, the degree is `3`.

## Run

    cargo run
