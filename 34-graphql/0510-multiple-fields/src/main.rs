use async_graphql::*;

#[derive(SimpleObject)]
struct User {
    name: String,
    age: i32,
}

struct Query;

#[Object]
impl Query {
    async fn user(&self) -> User {
        User {
            name: "alice".to_string(),
            age: 30,
        }
    }
}

#[tokio::main]
async fn main() {
    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
    let res = schema.execute("{ user { name age } }").await;
    let v = res.data.into_json().unwrap();
    println!("{}", v["user"]["name"].as_str().unwrap());
    println!("{}", v["user"]["age"].as_i64().unwrap());
}
