use axum::{Router, routing::get, body::Body};
use axum::http::Request;
use tower::ServiceExt;
use tower_http::catch_panic::CatchPanicLayer;

// The handler throws a real error (panics). It never returns normally.
async fn boom() -> &'static str {
    panic!("kaboom");
}

#[tokio::main]
async fn main() {
    // `CatchPanicLayer` is the framework's error-handling middleware: it catches
    // the panic thrown by the handler and turns it into a 500 response.
    let app = Router::new()
        .route("/boom", get(boom))
        .layer(CatchPanicLayer::new());

    // Exercise the route IN-PROCESS via tower's `oneshot` (no network port).
    let res = app
        .oneshot(
            Request::builder()
                .uri("/boom")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Print the real status code from the framework's HTTP response.
    println!("{}", res.status().as_u16());
}
