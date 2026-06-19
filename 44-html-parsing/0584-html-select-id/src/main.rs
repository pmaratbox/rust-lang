use scraper::{Html, Selector};

fn txt(el: scraper::ElementRef) -> String {
    el.text().collect::<String>()
}

fn main() {
    let doc = Html::parse_document(
        r#"<html><body>
<h1>Hello</h1>
<span id="status">active</span>
<ul class="items">
<li class="item">apple</li>
<li class="item">banana</li>
<li class="item">cherry</li>
</ul>
<a href="https://example.com">site</a>
<div class="content"><p>first</p><p>second</p></div>
</body></html>"#,
    );

    // CSS id selector `#status` matches the element whose id is "status".
    let sel = Selector::parse("#status").unwrap();
    let el = doc.select(&sel).next().unwrap();
    println!("{}", txt(el));
}
