# 0546 — Toggle

The [`rust-fsm`](https://crates.io/crates/rust-fsm) crate's `state_machine!`
macro declares a finite state machine as a transition table: each row maps a
`(state, input)` pair to a next state. Here the `Toggle` machine has two states,
`Off` and `On`, and a single `Toggle` input that flips between them. Starting
from `Off`, we feed the `Toggle` event three times (off -> on -> off -> on) via
`consume`, then print the machine's resulting state lowercased.

## Run

    cargo run
