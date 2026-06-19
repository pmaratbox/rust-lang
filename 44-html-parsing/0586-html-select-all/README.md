# 0586 — Select All Elements by Class

Use the `scraper` crate to parse a fixed HTML document with `Html::parse_document`,
then query every element matching the CSS class selector `.item`. We collect each
matched element's text via `text()`, gather the results into a `Vec`, and join them
with commas to print `apple,banana,cherry`.

## Run

    cargo run
