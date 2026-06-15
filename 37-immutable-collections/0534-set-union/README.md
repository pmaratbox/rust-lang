# 0534 — Set union

The `im` crate provides persistent, immutable collections. Calling `HashSet::union` returns a brand-new set containing every element of both operands while leaving the originals untouched (structural sharing via cheap clones). We union `{1,2,3}` and `{3,4,5}`, then sort the result for deterministic output.

## Run

    cargo run
