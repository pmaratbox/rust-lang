use axum::{Router, routing::get, body::Body, http::HeaderMap};
use axum::http::Request;
use tower::ServiceExt;

async fn whoami(headers: HeaderMap) -> String {
    headers
        .get("X-Name")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default()
        .to_string()
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/whoami", get(whoami));
    let res = app
        .oneshot(
            Request::builder()
                .uri("/whoami")
                .header("X-Name", "alice")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
    println!("{}", String::from_utf8(bytes.to_vec()).unwrap());
}
