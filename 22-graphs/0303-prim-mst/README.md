# 0303 — Prim MST

Compute the MST total weight of the chain (0,1,1),(1,2,2),(2,3,3) with Prim, printing `6`. A `BinaryHeap` of `Reverse((weight, node))` always extracts the cheapest crossing edge.

## Run

    cargo run
