# 0209 — Observer

Notify two observers of a new value 5; each prints its id and the value on its own line. The subject keeps a `Vec<Box<dyn Observer>>` and calls `update` on each.

## Run

    cargo run
