# 0578 — Write CSV

Use the `csv` crate's `Writer` to emit two rows — the header `["name", "age"]`
and the data row `["Alice", "30"]` — into an in-memory buffer. We recover the
written bytes with `into_inner()`, normalize the crate's `\r\n` line endings to
`\n`, strip the trailing newline, and print the resulting CSV text
`name,age\nAlice,30`.

## Run

    cargo run
