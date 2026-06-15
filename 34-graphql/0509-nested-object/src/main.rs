use async_graphql::*;

#[derive(SimpleObject)]
struct Address {
    city: String,
}

#[derive(SimpleObject)]
struct User {
    address: Address,
}

struct Query;

#[Object]
impl Query {
    async fn user(&self) -> User {
        User {
            address: Address {
                city: "oslo".to_string(),
            },
        }
    }
}

#[tokio::main]
async fn main() {
    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
    let res = schema.execute("{ user { address { city } } }").await;
    let v = res.data.into_json().unwrap();
    println!("{}", v["user"]["address"]["city"].as_str().unwrap());
}
