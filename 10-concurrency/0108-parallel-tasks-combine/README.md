# 0108 — Parallel Tasks Combined

Run two independent tasks that produce 10 and 20 concurrently, then combine (sum) their results into `30`. Two `thread::spawn` tasks run in parallel and their joined return values are summed.

## Run

    cargo run
