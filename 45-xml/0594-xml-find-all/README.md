# 0594 — Find all elements

Use the `quick-xml` crate (with its `serde` deserializer) to parse the fixed
catalog document. We model `<catalog>` as a struct holding `book: Vec<Book>`,
so deserializing collects every `<book>` element. We then take each book's
`<title>` text and join them with commas, printing `Go,Rust`.

## Run

    cargo run
