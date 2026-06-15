# 0512 — Query variables

Defines a GraphQL schema with `async-graphql` whose `Query.item(id: Int!)` resolver returns an `Item { id }`. The query `query($id: Int!) { item(id: $id) { id } }` is executed in-process (no HTTP server) using **query variables**: the `$id` value is passed through the execution's `Variables` map (`{ id: 42 }`) rather than interpolated into the query string. The resolved `data.item.id` is extracted from the result and printed: `42`.

## Run

    cargo run
