# 0628 — Log a string field

Uses the `tracing` crate with `tracing-subscriber`'s JSON formatter to emit an INFO record `login` carrying one structured string field `user=alice`. The record is captured in memory through a shared `Vec<u8>` writer (timestamps disabled via `without_time()`), then parsed with `serde_json` and printed as a normalized `level|message|key=value` line: `info|login|user=alice`.

## Run

    cargo run
