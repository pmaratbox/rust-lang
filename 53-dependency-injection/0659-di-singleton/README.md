# 0659 — Singleton lifetime

The `shaku` DI container registers `RepoImpl` as a singleton `Repo` component inside `AppModule`. Resolving the interface twice with `resolve_ref` returns references to the one instance the module owns, so comparing the two trait-object pointers for identity prints `true`.

## Run

    cargo run
