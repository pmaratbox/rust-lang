# 0438 — Create table & insert

Open an in-memory SQLite database with the `rusqlite` driver (bundled SQLite), create a `users(id integer, name text)` table, insert three rows with bound parameters, then run `select name from users order by id` via a prepared statement and print each name on its own line.

## Run

    cargo run
