# 0658 — Inject a dependency

Uses the `shaku` DI container. Two components are registered in an
`AppModule`: `RepoImpl` (bound to the `Repo` interface, `data()` returns
`data`) and `ServiceImpl` (bound to the `Service` interface). `ServiceImpl`
declares `#[shaku(inject)] repo: Arc<dyn Repo>`, so the container wires the
`Repo` into it. We resolve `Service` from the container and call `run()`,
which delegates to the injected `repo.data()`, printing `data`.

## Run

    cargo run
