# 0710 — UUID version

Uses the `uuid` crate to parse the canonical UUID
`550e8400-e29b-41d4-a716-446655440000` and read its version number with
`get_version_num`. The version (the high nibble of the seventh byte) is
extracted by the library, not hardcoded — here it reports `4`.

## Run

    cargo run
