# 0022 — Concurrency

Start two tasks that produce `1` and `2`, let them run concurrently, then join their results and print `sum: 3`. `thread::spawn` starts an OS thread running the closure, and `join()` blocks until it finishes and returns its result wrapped in a `Result` (`.unwrap()` here). Ownership rules give each thread its own data, so there is no shared-memory race to guard.

## Run

    cargo run
