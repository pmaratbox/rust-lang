# 0579 — Extract a column

Use the `csv` crate to parse the fixed CSV text
`name,age,city\nAlice,30,Paris\nBob,25,London\nCarol,35,Berlin\n`. A
`ReaderBuilder` with `has_headers(false)` exposes every line as a
`StringRecord`, so row 0 is the header. We look up the index of the `age`
column by name in the header, pull that field out of each remaining data row,
and join the values with commas to print `30,25,35`.

## Run

    cargo run
