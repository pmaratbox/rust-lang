# 0306 — Reconstruct Shortest Path

On the Dijkstra graph 0->1(4),0->2(1),2->1(2),1->3(1),2->3(5), print the actual shortest path from 0 to 3 `0 2 1 3`. A `prev` vector recorded during relaxation lets us backtrack and then `reverse` the path.

## Run

    cargo run
