# 0661 — Multiple dependencies

Uses the `shaku` DI container to register a service graph where `ServiceImpl`
declares two injected dependencies (`Arc<dyn A>` and `Arc<dyn B>`) via
`#[shaku(inject)]`. The module wires up `AImpl`, `BImpl`, and `ServiceImpl`, and
`m.resolve_ref::<dyn Service>()` builds the whole graph. `A::x()` returns `a`,
`B::y()` returns `b`, and `run()` concatenates them to print `ab`.

## Run

    cargo run
