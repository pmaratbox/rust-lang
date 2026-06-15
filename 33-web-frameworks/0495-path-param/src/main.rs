use axum::{Router, routing::get, body::Body, extract::Path};
use axum::http::Request;
use tower::ServiceExt;

// The handler captures the `{id}` path segment via axum's `Path` extractor
// and echoes it straight back as the response body.
async fn show(Path(id): Path<String>) -> String {
    id
}

#[tokio::main]
async fn main() {
    // Build the real axum app with a path-parameter route.
    let app = Router::new().route("/users/:id", get(show));

    // Exercise the route IN-PROCESS via tower's oneshot (no port bound).
    let res = app
        .oneshot(Request::builder().uri("/users/42").body(Body::empty()).unwrap())
        .await
        .unwrap();

    // Print the body the framework built from the captured path parameter.
    let bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
    println!("{}", String::from_utf8(bytes.to_vec()).unwrap());
}
