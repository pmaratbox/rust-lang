# 0601 — Italic

Render italic (em) text with the `pulldown-cmark` crate. We parse the Markdown
`*italic*` with `Parser::new` and emit HTML via `html::push_html`, which wraps
the emphasized span in `<em>` inside a paragraph. The renderer appends a trailing
newline, so we strip it with `trim_end_matches('\n')` before printing.

## Run

    cargo run
