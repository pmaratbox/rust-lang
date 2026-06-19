# 0603 — Inline code

Render Markdown to HTML with the `pulldown-cmark` crate. We feed the
backtick-wrapped span `` `code` `` through a `Parser` and emit HTML with
`html::push_html`, which produces a `<code>` element inside a paragraph. The
trailing newline the renderer appends is stripped, leaving
`<p><code>code</code></p>`.

## Run

    cargo run
