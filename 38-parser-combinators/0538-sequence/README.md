# 0538 — Sequence

This lesson uses the `nom` crate's `tuple` sequence combinator to run two parsers one after the other. `char('a')` is applied first, then `char('b')`; `tuple` succeeds only if both match in order, returning a pair of the matched characters. `map` then combines that pair into a single `String`. Running the parser on the fixed input `"ab"` yields `ab`, which is printed.

## Run

    cargo run
