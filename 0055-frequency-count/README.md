# 0055 — Frequency Count

Count how many times each letter appears in `banana` and print the per-letter counts in alphabetical order: `a:3 b:1 n:2`. A `BTreeMap` keeps its keys in sorted order, so iterating it yields alphabetical output directly; `entry(ch).or_insert(0)` initializes a missing count.

## Run

    cargo run
