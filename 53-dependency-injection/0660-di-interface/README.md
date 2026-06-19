# 0660 — Bind interface to impl

Uses the `shaku` DI container to bind the `Animal` interface (a `shaku::Interface`
trait) to a `Dog` component via `#[shaku(interface = Animal)]`. The `AppModule`
registers `Dog`, and `module.resolve_ref::<dyn Animal>()` resolves the service by
its interface — not its concrete type — then calls `sound()`, which prints `woof`.

## Run

    cargo run
