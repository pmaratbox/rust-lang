use axum::{Router, routing::get, body::Body};
use axum::http::Request;
use tower::ServiceExt;

async fn body_of(app: &Router, uri: &str) -> String {
    let res = app
        .clone()
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
    String::from_utf8(bytes.to_vec()).unwrap()
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "home" }))
        .route("/about", get(|| async { "about" }));

    println!("{}", body_of(&app, "/").await);
    println!("{}", body_of(&app, "/about").await);
}
