# 0582 — Filter rows

Use the `csv` crate to parse the fixed CSV text
`name,age,city\nAlice,30,Paris\nBob,25,London\nCarol,35,Berlin\n`. A
`ReaderBuilder` with `has_headers(false)` exposes every line as a
`StringRecord`, so row 0 is the header. We skip it and keep the data rows whose
`age` column parses to a number greater than 28 (Alice 30, Carol 35; Bob 25 is
excluded), then join the surviving `name` values with commas to print
`Alice,Carol`.

## Run

    cargo run
