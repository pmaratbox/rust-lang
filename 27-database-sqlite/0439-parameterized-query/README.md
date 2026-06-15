# 0439 — Parameterized query

Creates an in-memory SQLite database with a `users` table, inserts three rows, then runs `select name from users where id = ?1` with the value `2` supplied as a real bound parameter (never string-interpolated) using the `rusqlite` driver's prepared statement and `query_row`. It prints the matched name.

## Run

    cargo run
