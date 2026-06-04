# 0378 — Barrier Synchronization

Have 3 threads each arrive at a barrier before any proceeds, then print `all reached: 3`. Rust's `std::sync::Barrier` releases all threads only once `n` of them have called `wait`.

## Run

    cargo run
