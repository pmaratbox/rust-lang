# 0576 — CSV header row

Use the `csv` crate to parse the fixed CSV text
`name,age,city\nAlice,30,Paris\nBob,25,London\nCarol,35,Berlin\n`.
A `ReaderBuilder` with `has_headers(false)` treats row 0 as a normal record,
so the first `StringRecord` holds the header fields. Those fields are joined
with a pipe and printed: `name|age|city`.

## Run

    cargo run
