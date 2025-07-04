use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode},
    response::Response,
    routing::{get, post},
};
use tower::ServiceExt; // for `oneshot`

use main::handlers;

#[tokio::test]
async fn test_handler_returns_hello_world() {
    let app = Router::new().route("/", get(handlers::handler));

    let response: Response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    assert_eq!(body, "Hello, World!");
}

#[tokio::test]
async fn test_handler_returns_submit() {
    let app = Router::new().route("/submit", post(handlers::submit_handler));

    let response: Response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/submit")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    assert_eq!(body, "Form submitted successfully!");
}
