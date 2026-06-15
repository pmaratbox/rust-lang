# 0482 — Uppercase filter

Render a template with the `tera` crate. The template `{{ name | upper }}` pipes the context variable `name` (set to `alice`) through Tera's built-in `upper` filter, which uppercases the string. `Tera::one_off` parses and renders the template against the context in one call, producing `ALICE`.

## Run

    cargo run
