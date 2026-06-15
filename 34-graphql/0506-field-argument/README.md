# 0506 — Field argument

Defines a GraphQL schema with `async-graphql` whose `Query.greet(name: String!)` resolver takes a **field argument** and returns `"hello " + name`. The query `{ greet(name: "alice") }` is executed in-process (no HTTP server) and the resolved value is extracted from the result data and printed: `hello alice`.

## Run

    cargo run
