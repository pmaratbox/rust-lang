# 0590 — Count matches

This lesson uses the `scraper` crate (Rust's real HTML-parsing library) to parse a fixed HTML document and count how many elements match a CSS selector. We select with the class selector `.item` and use the iterator's `.count()` to get the number of matching `<li>` elements.

## Run

    cargo run
