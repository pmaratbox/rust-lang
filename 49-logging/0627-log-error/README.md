# 0627 — Log at error level

Uses the `tracing` structured-logging library with a `tracing-subscriber`
`fmt().json().without_time()` subscriber whose writer is a shared in-memory
buffer (no real timestamp is emitted). An ERROR record with the message `boom`
is logged, the captured JSON line is parsed (`level` top-level, message at
`fields.message`), and a normalized `level|message` line is printed.

## Run

    cargo run
