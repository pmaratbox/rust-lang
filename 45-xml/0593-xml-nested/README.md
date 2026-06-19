# 0593 — Nested element

Use the `quick-xml` crate (with its `serialize` feature) plus `serde` to
deserialize the fixed catalog document into `Catalog` / `Book` structs via
`quick_xml::de::from_str`. Each `<book>` nests a `<price>` element, which serde
maps to the `price` field. We read the first book's nested `<price>` text and
print it as the integer `30`.

## Run

    cargo run
