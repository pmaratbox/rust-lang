# 0327 — FlatMap

FlatMap [1,2,3] with x -> [x, x*10] and print the flattened result `1 10 2 20 3 30`. Rust's iterator `flat_map` maps each element to a sub-iterator and concatenates them in one pass.

## Run

    cargo run
