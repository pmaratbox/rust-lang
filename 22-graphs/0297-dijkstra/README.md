# 0297 — Dijkstra Shortest Paths

On the weighted digraph 0->1(4),0->2(1),2->1(2),1->3(1),2->3(5), print the shortest distances from node 0 `0 3 1 4`. Rust's `BinaryHeap` is a max-heap, so wrap entries in `Reverse` for a min-priority queue.

## Run

    cargo run
