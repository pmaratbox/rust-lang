# 0599 — Heading

Render the Markdown `# Hello` to HTML with the `pulldown-cmark` crate. A
`Parser` turns the source into an event stream, `html::push_html` writes the
HTML for the level-1 ATX heading, and we strip the trailing newline the renderer
appends before printing `<h1>Hello</h1>`.

## Run

    cargo run
