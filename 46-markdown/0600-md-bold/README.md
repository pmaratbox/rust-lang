# 0600 — Bold

Render Markdown to HTML with the `pulldown-cmark` crate. We feed `**bold**`
through a `Parser` and emit HTML with `html::push_html`, which produces a
`<strong>` element inside a paragraph. The trailing newline the renderer appends
is stripped, leaving `<p><strong>bold</strong></p>`.

## Run

    cargo run
