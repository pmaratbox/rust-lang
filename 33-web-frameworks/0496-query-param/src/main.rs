use axum::{Router, routing::get, body::Body, extract::Query};
use axum::http::Request;
use std::collections::HashMap;
use tower::ServiceExt;

async fn greet(Query(params): Query<HashMap<String, String>>) -> String {
    let name = params.get("name").cloned().unwrap_or_default();
    format!("hello {}", name)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/greet", get(greet));
    let res = app
        .oneshot(
            Request::builder()
                .uri("/greet?name=alice")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
    println!("{}", String::from_utf8(bytes.to_vec()).unwrap());
}
