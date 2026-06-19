# 0604 — Unordered list

Render Markdown to HTML with the `pulldown-cmark` crate. We feed the two-item
list `- a\n- b` through a `Parser` and emit HTML with `html::push_html`, which
produces a `<ul>` containing two `<li>` elements. The trailing newline the
renderer appends is stripped before printing.

## Run

    cargo run
