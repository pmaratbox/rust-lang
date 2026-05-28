# 0009 — Maps

Build a `HashMap`, look up `"two"`, and print its value and the map's size.
`HashMap::from([...])` builds it from key/value tuples (here inferring
`HashMap<&str, i32>`). Indexing `m["two"]` panics on a missing key; `.get(key)`
returns `Option<&V>` for a safe lookup. `.len()` counts entries.

## Run

    cargo run
