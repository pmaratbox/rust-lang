# 0021 — Mutability & References

Have a function increment a value in place — through a pointer, reference, or mutable holder — so the caller sees it change from `before: 1` to `after: 2`. `&mut i32` is a unique mutable borrow, and `*n += 1` writes through it. The caller must opt in with `&mut n`, and the binding must be `let mut`. The borrow checker guarantees no other reference aliases `n` while this one is live.

## Run

    cargo run
