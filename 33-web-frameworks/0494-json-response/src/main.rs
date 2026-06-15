use axum::{Router, routing::get, body::Body, Json};
use axum::http::Request;
use serde::Serialize;
use tower::ServiceExt;

#[derive(Serialize)]
struct User {
    name: String,
}

async fn user() -> Json<User> {
    Json(User { name: "alice".to_string() })
}

#[tokio::main]
async fn main() {
    // Build the real axum app with a JSON route.
    let app = Router::new().route("/user", get(user));

    // Exercise the route IN-PROCESS via tower's oneshot (no port bound).
    let res = app
        .oneshot(Request::builder().uri("/user").body(Body::empty()).unwrap())
        .await
        .unwrap();

    // Print the JSON body produced by the framework's `Json` responder.
    let bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
    println!("{}", String::from_utf8(bytes.to_vec()).unwrap());
}
