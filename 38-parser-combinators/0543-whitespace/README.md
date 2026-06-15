# 0543 — Whitespace handling

The [`nom`](https://crates.io/crates/nom) crate builds parsers from small
combinators. Here `delimited(multispace0, digit1, multispace0)` wraps the
`digit1` integer parser between two `multispace0` parsers that consume optional
surrounding whitespace, discarding it and keeping only the inner result. Running
the combinator on `"  42  "` returns an `IResult`; we take `.unwrap().1` for the
parsed value and print it.

## Run

    cargo run
