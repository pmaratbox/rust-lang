# 0638 — Env override

Uses the `config` crate to layer two sources: the `config.json` file (`name` =
`myapp`) and an `Environment` source with the `APP` prefix. The in-process env
var `APP_NAME=from-env` is set before `build()`, and because the environment
source is added last it merges with higher priority than the file, overriding
`name`. The resolved value is printed: `from-env`.

## Run

    cargo run
