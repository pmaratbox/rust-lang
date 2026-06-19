use scraper::{Html, Selector};

fn txt(el: scraper::ElementRef) -> String {
    el.text().collect::<String>()
}

fn main() {
    // A bare <table> fragment — parse_fragment keeps it intact (parse_document
    // would wrap/normalize the markup around the table).
    let doc = Html::parse_fragment(
        "<table><tr><td>r1c1</td><td>r1c2</td></tr><tr><td>r2c1</td><td>r2c2</td></tr></table>",
    );
    // CSS `td` matches every cell; select() yields them in document (row-major) order.
    let cells = Selector::parse("td").unwrap();
    let out = doc
        .select(&cells)
        .map(txt)
        .collect::<Vec<_>>()
        .join(",");
    println!("{}", out); // r1c1,r1c2,r2c1,r2c2
}
