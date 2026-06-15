# 0547 — Traffic light

The [`rust-fsm`](https://crates.io/crates/rust-fsm) crate models a finite state
machine: the `state_machine!` macro declares the states and the
`State(Input) => Next` transitions. We define a traffic light that cycles
`red -> green -> yellow -> red` on each `Next` event. Starting in `Red`, we
`consume` two `Next` events (red -> green -> yellow) and print the resulting
state name lowercased. The final state comes from the machine, not a constant.

## Run

    cargo run
