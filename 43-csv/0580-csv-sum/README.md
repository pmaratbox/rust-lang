# 0580 — Sum a numeric column

Use the `csv` crate to parse the fixed `name,age,city` CSV text. With
`ReaderBuilder::has_headers(false)`, row 0 is the header and the data rows
follow. The `age` column (index 1) is parsed to `i64` for each data row and
summed (`30 + 25 + 35`) to print `90`.

## Run

    cargo run
