use async_graphql::*;

struct Query;

#[Object]
impl Query {
    async fn greet(&self, name: String) -> String {
        format!("hello {}", name)
    }
}

#[tokio::main]
async fn main() {
    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
    let res = schema.execute(r#"{ greet(name: "alice") }"#).await;
    let v = res.data.into_json().unwrap();
    println!("{}", v["greet"].as_str().unwrap());
}
