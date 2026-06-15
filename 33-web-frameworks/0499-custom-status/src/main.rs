use axum::{Router, routing::post, http::StatusCode, body::Body};
use axum::http::Request;
use tower::ServiceExt;

// Handler returns a custom status code (201 Created) instead of the default 200.
async fn create() -> StatusCode {
    StatusCode::CREATED
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/create", post(create));

    // Exercise the route IN-PROCESS via tower's `oneshot` (no network port).
    let res = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/create")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Print the real status code from the framework's HTTP response.
    println!("{}", res.status().as_u16());
}
