# 0206 — Facade

Expose a single facade call that starts three subsystems and reports `ready`. The `Facade` struct owns three subsystem structs and sequences their `init()` calls.

## Run

    cargo run
