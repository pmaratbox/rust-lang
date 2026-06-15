# 0527 — Timezone offset

Using the `chrono` crate, parse the fixed UTC instant `2026-06-15T12:00:00Z`, then convert it to a fixed `+05:00` zone with `FixedOffset::east_opt` and `with_timezone`. The library shifts the instant and we read back the local hour (`17`) via `Timelike::hour`, without relying on the OS timezone database.

## Run

    cargo run
