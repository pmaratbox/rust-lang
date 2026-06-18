# 0577 — Quoted CSV fields

Use the `csv` crate to parse the fixed CSV text
`name,note\nAlice,"hello, world"\n`. The data row's `note` field is wrapped in
double quotes so it can contain a literal comma. A `ReaderBuilder` with
`has_headers(false)` treats row 0 as the header and row 1 as the data row; the
reader's default quoting rules unwrap the quoted field into a single value, so
column 1 of the data row prints as `hello, world` (comma intact).

## Run

    cargo run
