# 0637 — Default for missing key

Uses the `config` crate to build a `Config` from `config.json` (a `File`
source plus an `Environment` source with prefix `APP`). The key `missing` is
not present in the file, so the default registered with
`set_default("missing", "fallback")` is resolved by `get_string("missing")`
and printed: `fallback`.

## Run

    cargo run
