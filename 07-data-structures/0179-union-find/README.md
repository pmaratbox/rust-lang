# 0179 — Union-Find

Union (0,1) and (2,3), then query connectivity of (0,1)=yes and (0,2)=no, printing `yes no`. A `Vec<usize>` parent array with recursive `find` plus path compression gives the disjoint-set.

## Run

    cargo run
