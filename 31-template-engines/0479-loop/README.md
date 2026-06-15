# 0479 — Loop

Iterate a list inside a template with the `tera` crate. The template uses Tera's `{% for n in nums %}...{% endfor %}` loop to walk the `nums = [1, 2, 3]` vector inserted into the `Context`, emitting each value. The `loop.last` flag drives a `{% if not loop.last %}\n{% endif %}` guard so a newline separates the items without leaving a trailing one, rendering `1`, `2`, `3` each on its own line.

## Run

    cargo run
