use axum::{Router, routing::get, body::Body};
use axum::http::Request;
use tower::ServiceExt;

#[tokio::main]
async fn main() {
    // A router that only defines GET / — any other path is unmatched.
    let app = Router::new().route("/", get(|| async { "hello" }));

    // Drive the router IN-PROCESS via tower's oneshot (no port bound).
    // Requesting an undefined route makes axum produce a real 404 response.
    let res = app
        .oneshot(
            Request::builder()
                .uri("/missing")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Print the actual status code from the HTTP response.
    println!("{}", res.status().as_u16());
}
