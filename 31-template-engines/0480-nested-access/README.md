# 0480 — Nested access

Uses the Tera template engine to render `{{ user.name }}` against the fixed data `{user:{name:alice}}`, inserted as a nested JSON object. Tera resolves the dotted path into the nested structure and prints `alice`.

## Run

    cargo run
