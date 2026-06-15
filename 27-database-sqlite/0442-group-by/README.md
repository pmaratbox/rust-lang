# 0442 — Group by

Open an in-memory SQLite database with the `rusqlite` driver (bundled SQLite), create a `sales(category text, amount integer)` table, insert five rows with bound parameters, then run `select category, sum(amount) from sales group by category order by category` via a prepared statement to aggregate amounts per category and print each result as `category sum` (space-separated).

## Run

    cargo run
