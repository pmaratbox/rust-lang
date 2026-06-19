# 0589 — Extract table cells

Use the `scraper` crate to parse a bare `<table>` document with
`Html::parse_fragment` (so the table markup is kept intact). We build a
`Selector` from the CSS tag selector `td`, which matches every table cell.
Iterating the matches with `select` yields them in document order (row-major),
and for each one we collect `text()` into a `String`. Joining the results with
commas prints `r1c1,r1c2,r2c1,r2c2`.

## Run

    cargo run
