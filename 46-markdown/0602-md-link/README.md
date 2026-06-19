# 0602 — Link

Render the inline link `[text](http://x.com)` to HTML with the `pulldown-cmark`
crate. `Parser::new` tokenizes the Markdown and `html::push_html` writes the
result, which we trim of its trailing newline before printing:
`<p><a href="http://x.com">text</a></p>`.

## Run

    cargo run
