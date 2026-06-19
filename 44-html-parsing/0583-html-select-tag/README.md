# 0583 — Select by tag

Parse the shared HTML document with the `scraper` crate, then query it with the
CSS type selector `h1`. We take the first matching element, collect its text
nodes with `.text()`, and print the result `Hello`.

## Run

    cargo run
