# 0587 — Read an attribute

Parse the shared HTML document with the `scraper` crate, then query it with the
CSS type selector `a`. We take the first matching element and read its `href`
attribute via `.value().attr("href")`, printing `https://example.com`.

## Run

    cargo run
