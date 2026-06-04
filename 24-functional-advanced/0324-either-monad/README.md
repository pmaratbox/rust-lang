# 0324 — Either Monad

Chain Either computations: a successful divide chain yields 2, and a divide-by-zero yields an error, printing `2 err`. Rust's `Result` is the Either monad, where `and_then` short-circuits on the first `Err`.

## Run

    cargo run
