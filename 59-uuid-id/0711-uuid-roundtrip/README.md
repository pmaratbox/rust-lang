# 0711 — Parse and format

Uses the `uuid` crate's `Uuid::parse_str` to parse the uppercase string
`550E8400-E29B-41D4-A716-446655440000` into a `Uuid`, then prints it via
its `Display` implementation. Parsing is case-insensitive, while the
canonical rendering is always lowercase, so the round-trip normalizes the
input to `550e8400-e29b-41d4-a716-446655440000`.

## Run

    cargo run
