# 0657 — Resolve a service

Uses the `shaku` DI container. We register a `Greeter` component (bound to the
`Greeter` interface trait) inside a `module!`, build the container, and then
`resolve_ref::<dyn Greeter>()` to pull the service back out. Calling `greet()`
on the resolved instance returns `hello`, which we print.

## Run

    cargo run
