# 0402 — Observer Contract

Demonstrate the observer contract next*-then-terminal: emit 1 and 2, complete, and show that a post-complete next is ignored. A `stopped` bool guarded by early `return` keeps the methods idiomatic and panic-free in Rust.

## Run

    cargo run
