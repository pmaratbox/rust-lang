# 0220 — Null Object

Compare a no-op null logger with a real logger; only the real one records, so print the logged count `1`. A `NullLogger` implements the `Logger` trait with an empty `log()`.

## Run

    cargo run
