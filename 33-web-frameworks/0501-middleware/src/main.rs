use axum::{
    Router,
    routing::get,
    body::Body,
    http::Request,
    response::Response,
    middleware::{self, Next},
};
use tower::ServiceExt;

// Middleware: runs the inner handler, then rewrites the response body by
// prefixing it with `[mw] `. The handler itself only returns "hello".
async fn prefix_middleware(req: Request<Body>, next: Next) -> Response {
    let res = next.run(req).await;
    let (parts, body) = res.into_parts();
    let bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();
    let mut new_body = b"[mw] ".to_vec();
    new_body.extend_from_slice(&bytes);
    Response::from_parts(parts, Body::from(new_body))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "hello" }))
        .layer(middleware::from_fn(prefix_middleware));

    // Exercise the route IN-PROCESS via tower's oneshot (no port bound).
    let res = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
    println!("{}", String::from_utf8(bytes.to_vec()).unwrap());
}
