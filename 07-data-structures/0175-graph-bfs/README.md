# 0175 — Graph BFS

Breadth-first traverse from node 0 of the graph 0:[1,2] 1:[0,3] 2:[0,3] 3:[1,2], printing visit order `0 1 2 3`. A `VecDeque` as the FIFO frontier plus a `visited` bool vector gives the canonical BFS.

## Run

    cargo run
