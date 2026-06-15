use async_graphql::*;

#[derive(SimpleObject)]
struct Item {
    id: i32,
}

struct Query;

#[Object]
impl Query {
    async fn item(&self, id: i32) -> Item {
        Item { id }
    }
}

#[tokio::main]
async fn main() {
    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);

    // The `$id` variable is supplied via the execution's variables map,
    // not interpolated into the query string.
    let variables = Variables::from_json(serde_json::json!({ "id": 42 }));
    let request = Request::new("query($id: Int!) { item(id: $id) { id } }")
        .variables(variables);

    let res = schema.execute(request).await;
    let v = res.data.into_json().unwrap();
    println!("{}", v["item"]["id"].as_i64().unwrap());
}
