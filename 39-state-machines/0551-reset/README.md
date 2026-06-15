# 0551 — Reset

The [`rust-fsm`](https://crates.io/crates/rust-fsm) crate's `state_machine!`
macro defines a finite state machine declaratively. Here a `Process` starts
`Idle`, moves to `Running` on `Start`, and a `Reset` event transitions it back
to the initial `Idle` state. After firing `Start` then `Reset`, the machine's
own state is read back and printed lowercased.

## Run

    cargo run
