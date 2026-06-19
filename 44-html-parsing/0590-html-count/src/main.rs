use scraper::{Html, Selector};

fn main() {
    let doc = Html::parse_document(r#"<html><body>
<h1>Hello</h1>
<span id="status">active</span>
<ul class="items">
<li class="item">apple</li>
<li class="item">banana</li>
<li class="item">cherry</li>
</ul>
<a href="https://example.com">site</a>
<div class="content"><p>first</p><p>second</p></div>
</body></html>"#);

    let sel = Selector::parse(".item").unwrap();
    let count = doc.select(&sel).count();
    println!("{}", count);
}
