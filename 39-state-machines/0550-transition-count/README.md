# 0550 — Transition count

A finite state machine (FSM) built with the `rust-fsm` crate's `state_machine!` macro, using its *output actions* to count transitions. Each edge declares a `[Tick]` output that the machine emits whenever that transition fires — a per-transition action/callback. A three-step workflow (`Pending --start--> Running --work--> Working --finish--> Done`) fires a fixed sequence of three valid events; every emitted `Tick` increments a counter, so the resulting count of `3` comes from the machine's actions, never hardcoded.

## Run

    cargo run
