use async_graphql::*;

struct Query;

#[Object]
impl Query {
    async fn hello(&self) -> &str {
        "world"
    }
}

#[tokio::main]
async fn main() {
    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
    let res = schema.execute("{ hello }").await;
    let data = res.data.into_json().unwrap();
    println!("{}", data["hello"].as_str().unwrap());
}
