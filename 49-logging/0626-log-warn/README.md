# 0626 — Log at warn level

Uses the `tracing` crate with a `tracing-subscriber` JSON formatter to emit a
WARN-level record with the message `low disk`. The subscriber is configured with
`.json().without_time()` and a custom in-memory writer, so the record is captured
into a shared buffer (no real timestamp, nothing printed to the terminal). The
captured JSON line is then parsed and normalized: the `WARN` level is lowercased
to `warn` and joined with the message as `warn|low disk`.

## Run

    cargo run
