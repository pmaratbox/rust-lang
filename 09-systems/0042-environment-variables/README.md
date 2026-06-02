# 0042 — Environment Variables

Read the environment variable `LESSON_ENV_VAR`, falling back to `default` when it is unset, and print `value: default`. `std::env::var` returns a `Result` — `Err` when the variable is unset (or not valid UTF-8) — so `unwrap_or_else` supplies the default. `var_os` instead returns an `Option<OsString>`, accepting values that are not valid UTF-8.

## Run

    cargo run
