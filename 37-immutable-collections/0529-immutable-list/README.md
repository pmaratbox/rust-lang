# 0529 — Immutable list

The [`im`](https://crates.io/crates/im) crate provides persistent data
structures. `im::Vector` shares structure across clones, so `clone()` is cheap
and `push_back` on the clone produces a NEW list while the original stays
unchanged. We print the new list, then the untouched original.

## Run

    cargo run
