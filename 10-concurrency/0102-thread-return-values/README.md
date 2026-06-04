# 0102 — Threads Returning Values

Run two threads that compute the squares of 3 and 4, join them, and print the sum of their results `25`. A thread's closure return value flows out through `JoinHandle::join`, which yields it as a `Result`.

## Run

    cargo run
