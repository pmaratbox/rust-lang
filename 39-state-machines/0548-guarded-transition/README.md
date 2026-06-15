# 0548 — Guarded transition

The [`rust-fsm`](https://crates.io/crates/rust-fsm) crate's `state_machine!`
macro defines states and transitions declaratively. A *guarded transition* is
one that is only valid from a specific state: here the `Open` event is listed
only for the `Unlocked` state, so the transition table itself guards it — firing
`Open` from `Locked` would be rejected. Starting in `Locked`, we fire `Unlock`
then `Open` and print the resulting state name lowercased.

## Run

    cargo run
