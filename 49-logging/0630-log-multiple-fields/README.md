# 0630 — Log multiple fields

Uses the `tracing` library with `tracing-subscriber`'s JSON formatter
(`fmt().json().without_time()`) to emit one INFO record `request` carrying two
structured fields, `method="GET"` (string) and `status=200` (int). The
subscriber writes JSON lines to a shared in-memory buffer (no real timestamp);
the program parses each line with `serde_json`, normalizes the level, sorts the
fields by key, and prints `info|request|method=GET|status=200`.

## Run

    cargo run
