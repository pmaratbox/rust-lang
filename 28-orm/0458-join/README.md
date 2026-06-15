# 0458 — Join

Use the `diesel` query builder over an in-memory SQLite database (with the bundled `libsqlite3-sys`) to join two tables. It declares `users` and `posts` with `diesel::table!`, links them with `diesel::joinable!`, inserts the rows through `insert_into`/`Insertable`, then runs `posts::table.inner_join(users::table)` selecting `(users::name, posts::title)` ordered by name then title, loading the joined tuples and printing each as `name title`.

## Run

    cargo run
