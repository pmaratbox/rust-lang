# 0346 — Newtype Wrapper

Wrap raw integers in distinct UserId and ProductId types so they cannot be confused, printing `user-1 prod-2`. Rust's tuple-struct newtype pattern gives each ID a separate compile-time type.

## Run

    cargo run
