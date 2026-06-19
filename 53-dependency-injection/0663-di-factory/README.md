# 0663 — Factory provider

Uses the `shaku` DI container. Instead of registering a long-lived singleton
`Component`, we register a `Provider` — shaku's factory mechanism. The
`#[derive(Provider)]` macro generates a `provide` function that *builds* a
fresh `Widget` on demand. We register the factory under `providers`, build the
container, then call `module.provide::<dyn Widget>()` to have the factory
construct the object. Calling `value()` on the result returns `built`, which we
print.

## Run

    cargo run
