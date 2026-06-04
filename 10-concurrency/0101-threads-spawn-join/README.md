# 0101 — Threads: Spawn and Join

Spawn 3 worker threads, wait for all of them to finish, then print `done: 3`. `thread::spawn` returns a `JoinHandle` whose `join()` blocks until the thread completes.

## Run

    cargo run
