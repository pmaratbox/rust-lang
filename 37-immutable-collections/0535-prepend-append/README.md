# 0535 — Prepend & append

This lesson uses the `im` crate's persistent `Vector`. Starting from the immutable list `[2, 3]`, `push_front` prepends `1` and `push_back` appends `4`. Each update runs on a `clone` (cheap thanks to structural sharing) and returns a new list, leaving the original `[2, 3]` unchanged. The final list is printed space-joined.

## Run

    cargo run
