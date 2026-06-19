# 0598 — Filter by attribute

Parse the shared catalog document with the `quick-xml` crate (using its serde
`from_str` deserializer). Each `<book>` deserializes into a struct whose `@lang`
attribute and `<title>` text are mapped via `#[serde(rename)]`. We keep only the
books whose `lang` attribute equals `en`, extract their titles, and join them
with commas, printing `Go`.

## Run

    cargo run
