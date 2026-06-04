# 0330 — Lens Get/Set

Use a lens over the nested value {a:{b:1}} to get b (1) and to set b to 2, printing `1 2`. The `Lens` struct pairs a getter and an immutable setter, so `set` clones the struct and returns a fresh copy with `b` updated.

## Run

    cargo run
