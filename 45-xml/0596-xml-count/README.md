# 0596 — Count elements

Uses the `quick-xml` library (with `serde` derive) to deserialize the fixed catalog document into structs, then counts the `<book>` elements by taking the length of the deserialized `Vec<Book>` and prints it.

## Run

    cargo run
