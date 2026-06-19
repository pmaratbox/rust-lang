# 0632 — Level filtering

Uses the `tracing` logging library with a `tracing-subscriber` JSON formatter
configured via `with_max_level(Level::WARN)` and a shared in-memory buffer as its
writer (no timestamp, `without_time()`). An `info!("hidden")` record is emitted
below the minimum level and is dropped before it ever reaches the buffer; a
`warn!("visible")` record passes the filter and is captured. The captured JSON
line is parsed (`serde_json`), its level normalized to `warn`, and printed as the
single line `warn|visible`.

## Run

    cargo run
