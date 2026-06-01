# 0026 — Sets

Build a set from `1, 2, 2, 3` so the duplicate collapses, then print its `size: 3` and whether it contains `2` (`has 2: yes`) and `5` (`has 5: no`). `HashSet<i32>` stores unique values with average O(1) `insert`/`contains`; building it with `HashSet::from([...])` drops the duplicate. `.len()` reports the count and `.contains` takes a reference. `BTreeSet` is the ordered alternative.

## Run

    cargo run
