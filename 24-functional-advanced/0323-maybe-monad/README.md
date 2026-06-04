# 0323 — Maybe Monad

Chain Maybe operations: Some(2) then +3 then *2 gives 10, and a None chain yields the fallback, printing `10 none`. Rust's `Option` is the Maybe monad, with `and_then` as monadic bind and `unwrap_or_else` for the fallback.

## Run

    cargo run
