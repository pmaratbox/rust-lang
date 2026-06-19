# 0631 — Deterministic field order

Uses the `tracing` crate with `tracing-subscriber`'s JSON formatter. An INFO
record `metric` is emitted with two structured fields in non-alphabetical order
(`zeta = 2` then `alpha = 1`). The subscriber writes JSON (with `.without_time()`
so there is no real timestamp) into a shared in-memory buffer. The captured
record is parsed with `serde_json`, the level is normalized to the short
lowercase set, and the fields are sorted by key before printing
`level|message|key=value...` — yielding `info|metric|alpha=1|zeta=2`.

## Run

    cargo run
