use axum::{Router, body::{Body, to_bytes}, http::{Request, StatusCode}, routing::get, response::{Html, Response}};
use tower::ServiceExt; // for `oneshot`

async fn handler() -> Html<&'static str> {
    Html("Hello, World!")
}

#[tokio::test]
async fn test_handler_returns_hello_world() {
    let app = Router::new().route("/", get(handler));

    let response: Response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    assert_eq!(body, "Hello, World!");
}
 