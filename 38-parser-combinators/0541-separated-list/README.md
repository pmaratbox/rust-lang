# 0541 — Separated list

The [`nom`](https://crates.io/crates/nom) parser-combinator library builds
parsers by composing small combinators. Here `separated_list1(char(','), integer)`
parses one-or-more integers separated by `,` into a `Vec<i32>`, where each
integer comes from `map(digit1, ...)`. We run it on the fixed input `"1,2,3"`,
sum the parsed list, and print the total.

## Run

    cargo run
