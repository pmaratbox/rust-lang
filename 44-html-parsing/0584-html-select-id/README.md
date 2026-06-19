# 0584 — Select by id

Use the `scraper` crate to parse the fixed HTML document and query it with the
CSS id selector `#status`. `Html::parse_document` builds a DOM, `Selector::parse("#status")`
matches the single element whose `id` attribute is `status`, and collecting that
element's `text()` yields `active`.

## Run

    cargo run
