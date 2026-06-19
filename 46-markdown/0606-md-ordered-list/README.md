# 0606 — Ordered list

This lesson uses the [`pulldown-cmark`](https://crates.io/crates/pulldown-cmark) crate to render an ordered list (`1. a\n2. b`) from Markdown to HTML. The parser emits an `<ol>` element with `<li>` items; the trailing newline appended by the renderer is stripped before printing.

## Run

    cargo run
