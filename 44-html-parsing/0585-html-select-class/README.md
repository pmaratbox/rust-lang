# 0585 — Select by class

Use the `scraper` crate to parse the fixed HTML document with
`Html::parse_document`. We build a `Selector` from the CSS class selector
`.item`, call `select` to iterate matching elements, take the first one with
`next()`, and collect its `text()` into a `String`. The first `<li class="item">`
holds `apple`, so that is what we print.

## Run

    cargo run
