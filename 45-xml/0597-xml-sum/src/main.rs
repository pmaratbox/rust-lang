// Sum numeric children — quick-xml + serde deserialization.
use serde::Deserialize;

#[derive(Deserialize)]
struct Book {
    #[serde(rename = "@id")]
    #[allow(dead_code)]
    id: String,
    #[serde(rename = "@lang")]
    #[allow(dead_code)]
    lang: String,
    #[allow(dead_code)]
    title: String,
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
    println!("{}", c.book.iter().map(|b| b.price).sum::<i64>());
}
