# 0595 — All attributes

Use the `quick-xml` crate (with the `serialize` feature) together with `serde`
to deserialize the fixed catalog document. We model each `<book>` as a struct
whose `id` field maps to the XML `id` attribute via `#[serde(rename = "@id")]`,
then call `quick_xml::de::from_str`. We collect the `id` attribute of every
`<book>` in document order and join them with commas, producing `b1,b2`.

## Run

    cargo run
