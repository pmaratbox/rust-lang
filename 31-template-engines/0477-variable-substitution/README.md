# 0477 — Variable substitution

Use the `tera` crate (a Jinja2-style template engine) to render a fixed template that contains a `{{ name }}` variable substitution. The value `alice` is inserted into a `tera::Context`, and `Tera::one_off` parses and renders the template string `Hello {{ name }}` in one call, producing `Hello alice`.

## Run

    cargo run
