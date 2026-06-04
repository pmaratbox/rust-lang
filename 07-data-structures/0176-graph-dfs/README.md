# 0176 — Graph DFS

Depth-first traverse from node 0 of the graph 0:[1,2] 1:[0,3] 2:[0,3] 3:[1,2], printing visit order `0 1 3 2`. A small recursive helper carrying `&mut [bool]` visited threads the depth-first walk in neighbor order.

## Run

    cargo run
