# 0629 — Log an integer field

Uses the `tracing` crate with `tracing-subscriber`'s JSON formatter to emit an
INFO record `processed` carrying one integer structured field `count=5`. The
formatter is configured with `.without_time()` and writes to a shared in-memory
buffer, so no real timestamp leaks and nothing touches stdout/stderr. The
captured JSON line is parsed with `serde_json`: the level is normalized to
lowercase, the message is read from `fields.message`, and the remaining fields
are sorted by key and rendered as `|key=value` (integers as-is), producing
`info|processed|count=5`.

## Run

    cargo run
