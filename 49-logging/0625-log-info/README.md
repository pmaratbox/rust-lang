# 0625 — Log at info level

Uses the `tracing` structured-logging library with a `tracing-subscriber`
`fmt().json().without_time()` subscriber whose writer is an in-memory shared
buffer (so no real timestamp is recorded and nothing prints to the console).
An INFO record with the message `service started` and no fields is emitted, the
captured JSON line is parsed (level at the top level, message at
`fields.message`), and one normalized line is printed: `info|service started`.

## Run

    cargo run
