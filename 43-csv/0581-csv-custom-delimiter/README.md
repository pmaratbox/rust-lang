# 0581 — Custom delimiter

Use the `csv` crate to parse the semicolon-delimited text `a;b;c\n1;2;3\n` by
configuring the reader's delimiter to `;` with `ReaderBuilder::delimiter(b';')`.
The second (data) row's fields are taken and joined with commas to print
`1,2,3`.

## Run

    cargo run
