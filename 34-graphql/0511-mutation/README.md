# 0511 — Mutation

Defines a GraphQL schema with `async-graphql` that has a `Mutation` type whose `addUser(name: String!): User` resolver returns a `User` object. The **mutation** `mutation { addUser(name: "bob") { name } }` is executed in-process (no HTTP server) and the resolved value is extracted from the result data and printed: `bob`.

## Run

    cargo run
