# 0453 — Update a row

Open an in-memory SQLite database with the `diesel` ORM (schema declared via the `diesel::table!` macro, with the `bundled` SQLite engine through `libsqlite3-sys`), insert three `users` rows from an `#[derive(Insertable)]` struct, then modify a persisted entity with `diesel::update(...).set(...)` to change bob's age to 40. It reads the rows back through the query DSL — `filter(users::age.ge(35)).order(users::id).load::<User>` — and prints each surviving row as `name age`.

## Run

    cargo run
