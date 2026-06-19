# 0588 — Select nested elements

Use the `scraper` crate to parse the fixed HTML document with
`Html::parse_document`. We build a `Selector` from the CSS descendant selector
`.content p`, which matches every `<p>` element nested inside the element with
class `content`. We iterate the matches with `select`, collect each one's
`text()` into a `String`, and join the results with commas, printing
`first,second`.

## Run

    cargo run
