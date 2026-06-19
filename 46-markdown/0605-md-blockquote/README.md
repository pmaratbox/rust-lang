# 0605 — Blockquote

Use the `pulldown-cmark` crate to render a Markdown blockquote (`> quote`) to
HTML. The `> ` prefix produces a `<blockquote>` element that wraps the quoted
text in a paragraph. We strip the trailing newline the renderer appends before
printing.

## Run

    cargo run
