# 0264 — Turnstile FSM

Drive a turnstile (locked/unlocked) with events coin, push, push and print the resulting states `unlocked locked locked`. A `match` on a `(state, event)` tuple expresses the full transition table.

## Run

    cargo run
