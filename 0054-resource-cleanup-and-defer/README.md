# 0054 — Resource Cleanup & Defer

Acquire a resource, use it, and let the language release it automatically at scope exit, printing `open`, `use`, and `close` in that order. Cleanup is RAII: the `Drop` trait's `drop` runs automatically when `_resource` leaves scope at the end of `main`. There is no explicit free (and a bare `_` would drop it immediately).

## Run

    cargo run
