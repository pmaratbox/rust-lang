# 0308 — Strongly Connected Components

Count the strongly connected components of 0->1,1->2,2->0,2->3, printing `2`. Tarjan's algorithm lives in a small struct so the recursive DFS can borrow shared index/low/stack state.

## Run

    cargo run
