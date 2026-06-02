# 0056 — Linked List

Build a singly-linked list holding `1`, `2`, and `3`, then traverse it from head to tail and print `1 -> 2 -> 3`. Each node owns the next through `Option<Box<Node>>` (`Box` is a heap pointer, `None` the tail). Traversal borrows with `as_deref()`, turning `&Option<Box<Node>>` into `Option<&Node>`.

## Run

    cargo run
