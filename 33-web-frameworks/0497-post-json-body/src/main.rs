use axum::{Router, routing::post, body::Body, Json};
use axum::http::Request;
use serde::Deserialize;
use tower::ServiceExt;

#[derive(Deserialize)]
struct Pair {
    a: i64,
    b: i64,
}

// axum's `Json` extractor parses the request body into `Pair`.
async fn sum(Json(p): Json<Pair>) -> String {
    (p.a + p.b).to_string()
}

#[tokio::main]
async fn main() {
    // Build the real axum app with a POST route that parses a JSON body.
    let app = Router::new().route("/sum", post(sum));

    // Exercise the route IN-PROCESS via tower's oneshot (no port bound).
    let res = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/sum")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"a":2,"b":3}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    // Print the body produced by the handler from the parsed JSON.
    let bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
    println!("{}", String::from_utf8(bytes.to_vec()).unwrap());
}
