# 0544 — Expression

The [`nom`](https://crates.io/crates/nom) crate builds parsers by composing
combinators. Here `separated_list1(char('+'), integer)` parses a `+`-separated
sequence of integers, and `map` folds that `Vec<i32>` into its sum. We run the
parser on the fixed input `"10+20+30"` and print the result.

## Run

    cargo run
