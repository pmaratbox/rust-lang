# 0542 — Map / transform

The [`nom`](https://crates.io/crates/nom) crate builds parsers from small
combinators. Here `digit1` matches one-or-more ASCII digits, and the `map`
combinator transforms the matched value: it parses the slice into an `i32` and
multiplies it by two. Running the parser on the fixed input `"21"` yields the
`IResult`, from which we take `.unwrap().1` (the value `42`) and print it.

## Run

    cargo run
