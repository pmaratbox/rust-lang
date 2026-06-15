use axum::{Router, routing::get, body::Body};
use axum::http::Request;
use tower::ServiceExt;

#[tokio::main]
async fn main() {
    // Build the real axum app: GET / returns the text "hello".
    let app = Router::new().route("/", get(|| async { "hello" }));

    // Exercise the route in-process via tower's `oneshot` (no port bound).
    let res = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    // Print the body taken from the actual HTTP response.
    let bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
    println!("{}", String::from_utf8(bytes.to_vec()).unwrap());
}
