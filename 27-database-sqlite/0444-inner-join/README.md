# 0444 — Inner join

Open an in-memory SQLite database with the `rusqlite` driver (using the `bundled` SQLite feature), create `users` and `orders` tables, and insert rows into each. It then runs an inner `join` that matches each order to its user on `users.id = orders.user_id`, ordered by `name` then `item`, using `Connection::execute` for the parameterized inserts and `prepare` plus `query_map` to read and print each joined row as `name item`.

## Run

    cargo run
