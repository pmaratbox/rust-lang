# 0537 — Parse an integer

The [`nom`](https://crates.io/crates/nom) crate builds parsers from small
combinators. Here `digit1` matches one-or-more ASCII digits, and `map` converts
the matched slice into an `i32`. Running the combinator on the input `"42"`
returns an `IResult`; we take `.unwrap().1` for the parsed value and print it.

## Run

    cargo run
