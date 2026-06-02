# 0067 — Prefix Sums

Compute the running totals (prefix sums) of `1, 2, 3, 4` — each element added to the sum of all the previous ones — and print them: `1 3 6 10`. `Iterator::scan` threads a mutable `total` through the iterator, yielding each running sum.

## Run

    cargo run
