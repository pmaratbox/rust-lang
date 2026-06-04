# 0211 — Command (Undo)

Execute an AddCommand that takes a counter from 0 to 5, then undo it back to 0, printing `5 0`. A `Command` trait exposes `execute`/`undo` over a `&mut` counter.

## Run

    cargo run
