# 0552 — Workflow

A finite state machine (FSM) built with the `rust-fsm` crate's declarative `state_machine!` macro, modelling a multi-step approval workflow. The states flow `Idle --submit--> Pending --approve--> Approved`. Starting from `Idle`, the machine consumes a `Submit` event then an `Approve` event and ends in `approved` — the resulting state is read back from the machine and lowercased, never hardcoded.

## Run

    cargo run
