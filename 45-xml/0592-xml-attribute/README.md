# 0592 — Read an attribute

Use the `quick-xml` crate (with the `serialize` feature) together with `serde`
to deserialize the fixed catalog document. We model each `<book>` as a struct
whose `id` field maps to the XML `id` attribute via `#[serde(rename = "@id")]`,
then call `quick_xml::de::from_str`. We read the `id` attribute of the first
`<book>` and print it, producing `b1`.

## Run

    cargo run
