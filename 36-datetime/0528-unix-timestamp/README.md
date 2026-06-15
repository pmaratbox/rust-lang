# 0528 — Unix timestamp

The `chrono` crate parses the fixed UTC instant `2026-06-15T00:00:00Z`, then `DateTime<Utc>::timestamp` computes its Unix timestamp (epoch seconds since 1970-01-01T00:00:00Z): `1781481600`.

## Run

    cargo run
