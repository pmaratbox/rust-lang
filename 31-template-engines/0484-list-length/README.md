# 0484 — List length

Use the `tera` crate (a Jinja2-style template engine) to render the length of a list. The fixed data is `items = [1, 2, 3]`, inserted into a `tera::Context` as a `serde_json` array, and the fixed template `{{ items | length }}` applies Tera's built-in `length` filter to count the elements. `Tera::one_off` parses and renders the template in one call, printing `3`.

## Run

    cargo run
