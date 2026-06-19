# 0662 — Inject a value

Uses the `shaku` DI container. `ConfigImpl` has a plain (non-injected) `value`
field, which shaku exposes as a build-time parameter. We register the component
in a `module!`, build the container while supplying the constant value `v1` via
`with_component_parameters`, then `resolve_ref::<dyn Config>()` and call
`value()` to print the injected constant.

## Run

    cargo run
