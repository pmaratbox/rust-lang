# 0049 — Zip & Enumerate

Pair the letters `a, b, c` with the numbers `1, 2, 3` position by position, formatting each pair as `key=value` and printing `a=1 b=2 c=3`. `Iterator::zip` pairs two iterators, stopping at the shorter, and `map` over the `(k, n)` tuples builds the strings. Its enumerate counterpart, `enumerate`, pairs each item with its index.

## Run

    cargo run
