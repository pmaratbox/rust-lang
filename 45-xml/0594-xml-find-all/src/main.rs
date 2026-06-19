// Find all <book> elements, take each book's <title>, join with commas.
// Parsing is done by quick-xml's serde deserializer (no hand-rolled parsing).
use serde::Deserialize;

#[derive(Deserialize)]
struct Book {
    #[serde(rename = "@id")]
    #[allow(dead_code)]
    id: String,
    #[serde(rename = "@lang")]
    #[allow(dead_code)]
    lang: String,
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
    let titles: Vec<String> = c.book.iter().map(|b| b.title.clone()).collect();
    println!("{}", titles.join(","));
}
