# 0478 — Conditional

Use the `tera` crate (a Jinja2-style template engine) to render based on a condition. The template uses Tera's `{% if %} … {% else %} … {% endif %}` control block on the boolean `logged_in` value inserted into a `tera::Context`. With `logged_in = true` the engine renders the `if` branch (`welcome`); otherwise it would render the `else` branch (`guest`). Rendering is done with `Tera::one_off` on the fixed template string.

## Run

    cargo run
