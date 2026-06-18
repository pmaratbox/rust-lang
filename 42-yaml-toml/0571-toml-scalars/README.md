# 0571 — TOML scalars

Use the `toml` crate to parse the document `title = "demo"\nversion = 2\n`
into a `toml::Value`. Read the top-level `title` (a string) and `version`
(an integer) keys and print them space-joined as `demo 2`.

## Run

    cargo run
