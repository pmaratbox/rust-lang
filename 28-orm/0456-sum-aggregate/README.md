# 0456 — Sum aggregate

Define a `users` model (id, name, age) with Diesel's `table!` macro and `#[derive(Insertable)]`, open an in-memory SQLite database via `SqliteConnection::establish(":memory:")`, and insert three rows through `diesel::insert_into`. The query uses Diesel's query DSL aggregate helper — `users::table.select(diesel::dsl::sum(users::age)).first::<Option<i64>>()` — to sum the `age` column, printing the total.

## Run

    cargo run
