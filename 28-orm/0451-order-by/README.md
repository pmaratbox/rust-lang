# 0451 — Order by

Use the `diesel` query builder over an in-memory SQLite database (`SqliteConnection::establish(":memory:")` with the bundled `libsqlite3-sys`). The `users` table is declared with `diesel::table!`, and `#[derive(Insertable)]`/`#[derive(Queryable)]` structs map the rows. After inserting three users via `diesel::insert_into`, the query `users::table.order(users::age.asc())` sorts by age ascending through the DSL, and each name is printed in order.

## Run

    cargo run
