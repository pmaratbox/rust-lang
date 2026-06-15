use async_graphql::*;

struct Query;

#[Object]
impl Query {
    async fn numbers(&self) -> Vec<i32> {
        vec![1, 2, 3]
    }
}

#[tokio::main]
async fn main() {
    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
    let res = schema.execute("{ numbers }").await;
    let data = res.data.into_json().unwrap();
    for n in data["numbers"].as_array().unwrap() {
        println!("{}", n.as_i64().unwrap());
    }
}
