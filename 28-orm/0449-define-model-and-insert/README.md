# 0449 — Define model & insert

Use the `diesel` ORM/query-builder over an in-memory SQLite database (with the `bundled` libsqlite3-sys feature). The schema is described with `diesel::table!`, and Rust structs derive `Queryable` (for reading) and `Insertable` (for writing). Rows are added with `diesel::insert_into(users::table).values(...)` and read back via the query DSL `users::table.order(users::id).load::<User>(...)`, printing each user's name in id order.

## Run

    cargo run
