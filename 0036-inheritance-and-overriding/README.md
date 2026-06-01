# 0036 — Inheritance & Overriding

Define a base `Animal` with a `speak` method, a `Dog` that overrides it, and call both, printing `animal: some sound` and `dog: Woof`. Rust has no class inheritance. A trait with a *default method* (`Animal::speak`) plays the base role; `Generic` accepts the default with an empty `impl`, while `Dog` overrides it. Shared behavior comes from traits, not subclassing.

## Run

    cargo run
