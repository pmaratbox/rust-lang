# 0539 — Alternative

The [`nom`](https://crates.io/crates/nom) crate builds parsers from combinators.
Here `nom::branch::alt` expresses a choice between two parsers — `tag("cat")` OR
`tag("dog")`. `alt` tries each alternative in order and returns the first match.
We run it on the fixed input `"dog"`, so the first parser fails and the second
succeeds, yielding `dog`.

## Run

    cargo run
