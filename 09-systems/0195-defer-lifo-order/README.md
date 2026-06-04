# 0195 — Defer LIFO Order

Register three deferred actions printing 1, 2, 3 and show they run in last-in-first-out order `3 2 1`. Rust has no `defer`, so RAII `Drop` guards fill the role and drop in reverse declaration order.

## Run

    cargo run
