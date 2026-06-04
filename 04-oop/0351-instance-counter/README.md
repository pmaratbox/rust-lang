# 0351 — Instance Counter

Track how many instances of a class have been created; after building three, print `3`. In Rust a `static AtomicUsize` is incremented in the `new` constructor for thread-safe counting.

## Run

    cargo run
