# 0635 — List value

Uses the `config` crate to load the fixed `config.json` file via
`Config::builder().add_source(File::with_name("config.json"))`. The `hosts`
key holds the array `["a", "b", "c"]`; it is read with `get_array`, each
element is converted to a string, and the values are joined with commas to
print the single line `a,b,c`.

## Run

    cargo run
