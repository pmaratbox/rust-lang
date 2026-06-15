# 0545 — Basic transition

A finite state machine (FSM) built with the `rust-fsm` crate's declarative `state_machine!` macro. The classic turnstile has two states, `Locked` and `Unlocked`; inserting a coin transitions `Locked --coin--> Unlocked`. Starting from `Locked`, the machine consumes one `Coin` event and ends in `unlocked` — the resulting state is read back from the machine and lowercased, never hardcoded.

## Run

    cargo run
