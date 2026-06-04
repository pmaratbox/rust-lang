# 0216 — Mediator

Have colleague A send "hi" through a mediator to colleague B, which prints `B got: hi`. The mediator owns colleague B and routes A's message to its `receive`.

## Run

    cargo run
