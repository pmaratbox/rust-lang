use async_graphql::*;

#[derive(SimpleObject)]
struct User {
    name: String,
}

struct Query;

#[Object]
impl Query {
    async fn user(&self) -> User {
        User {
            name: "alice".to_string(),
        }
    }
}

#[tokio::main]
async fn main() {
    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
    let res = schema.execute("{ user { name } }").await;
    let v = res.data.into_json().unwrap();
    println!("{}", v["user"]["name"].as_str().unwrap());
}
