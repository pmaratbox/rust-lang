use async_graphql::*;

#[derive(SimpleObject)]
struct User {
    name: String,
}

struct Query;

#[Object]
impl Query {
    async fn ping(&self) -> &str {
        "pong"
    }
}

struct Mutation;

#[Object]
impl Mutation {
    async fn add_user(&self, name: String) -> User {
        User { name }
    }
}

#[tokio::main]
async fn main() {
    let schema = Schema::new(Query, Mutation, EmptySubscription);
    let res = schema
        .execute(r#"mutation { addUser(name: "bob") { name } }"#)
        .await;
    let v = res.data.into_json().unwrap();
    println!("{}", v["addUser"]["name"].as_str().unwrap());
}
