# 0549 — Invalid transition

A finite state machine (FSM) built with the `rust-fsm` crate's declarative `state_machine!` macro. The turnstile starts `Locked`, where only a `Coin` event has a defined transition. Firing `Push` from `Locked` has no matching rule, so the machine rejects it by returning `Err`; we catch and ignore that error rather than crashing, leaving the state unchanged. The resulting state is read back from the machine and lowercased — still `locked`, never hardcoded.

## Run

    cargo run
