// Rust — quick-xml with serde de. Per-lesson Cargo. Run: cargo run
// Attributes map via #[serde(rename="@id")].
use serde::Deserialize;

#[derive(Deserialize)]
struct Book {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@lang")]
    #[allow(dead_code)]
    lang: String,
    #[allow(dead_code)]
    title: String,
    #[allow(dead_code)]
    price: i64,
}

#[derive(Deserialize)]
struct Catalog {
    book: Vec<Book>,
}

fn main() {
    let doc = r#"<catalog>
  <book id="b1" lang="en"><title>Go</title><price>30</price></book>
  <book id="b2" lang="fr"><title>Rust</title><price>45</price></book>
</catalog>"#;
    let c: Catalog = quick_xml::de::from_str(doc).unwrap();
    // Read the `id` attribute of the first <book> -> b1
    println!("{}", c.book[0].id);
}
