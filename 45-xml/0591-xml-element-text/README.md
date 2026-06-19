# 0591 — Element text

Parse the shared XML catalog with the `quick-xml` crate (serde deserialization).
We map the document onto `Catalog`/`Book` structs, then read the text of the
first book's `<title>` element and print it: `Go`.

## Run

    cargo run
