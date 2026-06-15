# 0531 — Immutable set

The `im` crate provides a persistent `HashSet` whose `update` method returns a brand new set with the element added, while the original set stays unchanged through structural sharing. Here we build `{1, 2, 3}`, call `update(4)` to get a new set, and print the new set's size (`4`) followed by the untouched original's size (`3`).

## Run

    cargo run
