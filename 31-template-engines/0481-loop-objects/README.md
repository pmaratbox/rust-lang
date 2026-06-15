# 0481 — Loop over objects

Use the `tera` crate (a Jinja2-style template engine) to iterate over a list of objects. A fixed list of users (each a `{name, age}` JSON object) is inserted into a `tera::Context`, and the template uses a `{% for u in users %}...{% endfor %}` loop to render `name: age` on one line per user via `Tera::one_off`. Only the rendered output must match; the `{{ }}` / `{% %}` syntax is Tera-specific.

## Run

    cargo run
