# 0200 — Precondition Assert

Check a precondition arg>0: for 5 print `ok`, and for -1 report the failure `error: must be positive`, on two lines. Returning a `Result` and matching on it keeps the failure recoverable rather than panicking with `assert!`.

## Run

    cargo run
