use axum::{Router, routing::get, body::Body};
use axum::http::{Request, Method};
use tower::ServiceExt;

#[tokio::main]
async fn main() {
    // Same path /item handles two methods: GET -> `get`, POST -> `post`.
    let app = Router::new().route(
        "/item",
        get(|| async { "get" }).post(|| async { "post" }),
    );

    // Exercise POST /item in-process via tower's oneshot (no port bound).
    let res = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/item")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
    println!("{}", String::from_utf8(bytes.to_vec()).unwrap());
}
