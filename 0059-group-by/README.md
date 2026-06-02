# 0059 — Group By

Group the words `one`, `two`, `three` by their length and print each length with its words, in ascending order of length: `3:[one,two] 5:[three]`. A `BTreeMap` keeps the length keys sorted, so iteration is already ascending; `entry(len).or_default()` creates an empty `Vec` on first use.

## Run

    cargo run
