# 0307 — A* on a Grid

Find the shortest path length from (0,0) to (2,2) on an obstacle-free 3x3 grid (4-directional) with the Manhattan heuristic, printing `4`. A `BinaryHeap` keyed on `Reverse((f, g, cell))` orders the frontier by `f = g + h`.

## Run

    cargo run
