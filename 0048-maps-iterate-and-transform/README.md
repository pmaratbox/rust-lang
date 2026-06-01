# 0048 — Maps: Iterate & Transform

Build a map from letters to numbers (`a`->1, `b`->2, `c`->3), sum all its values, and print `sum: 6`. `HashMap::values()` returns an iterator over the values, and the `Iterator::sum` adapter folds them (the result type is annotated as `i32`). `keys`, `values`, and `iter` are the traversal methods.

## Run

    cargo run
