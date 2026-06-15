# 0483 — Default value

Use the `tera` crate (a Jinja2-style template engine) with its `default` filter to supply a fallback when a variable is missing. The `tera::Context` contains no `name`, so the template `{{ name | default(value="anonymous") }}`, rendered with `Tera::one_off`, falls back to `anonymous`.

## Run

    cargo run
