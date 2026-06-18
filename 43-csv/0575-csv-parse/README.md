# 0575 — Parse CSV rows

Use the `csv` crate to parse the fixed CSV text
`name,age,city\nAlice,30,Paris\nBob,25,London\nCarol,35,Berlin\n`. A
`ReaderBuilder` with `has_headers(false)` exposes every line as a
`StringRecord`, so row 0 is the header. We skip it, read the first column
(`name`) of each remaining data row, and join the values with commas to print
`Alice,Bob,Carol`.

## Run

    cargo run
