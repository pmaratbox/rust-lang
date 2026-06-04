# 0151 — Singleton

Obtain a singleton instance twice and confirm both references are the same object, printing `same: yes`. A `static OnceLock` lazily initializes one shared instance behind a `&'static` reference.

## Run

    cargo run
