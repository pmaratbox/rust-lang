# 0109 — Run-Once Initialization

Ensure an initializer runs exactly once even when several threads race to trigger it, printing `init count: 1`. `std::sync::Once::call_once` guarantees the init body executes a single time across all racing threads.

## Run

    cargo run
