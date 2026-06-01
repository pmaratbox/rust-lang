# 0050 — Stacks

Push `1`, `2`, and `3` onto a stack, then pop them all off and print them in last-in-first-out order: `3 2 1`. A `Vec` is a stack: `push` appends and `pop` returns `Option<T>` (`None` when empty), so a `while let Some(...)` drains it in LIFO order.

## Run

    cargo run
