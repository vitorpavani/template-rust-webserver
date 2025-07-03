use axum::response::Html;

pub async fn handler() -> Html<&'static str> {
    Html("Hello, World!")
}

pub async fn submit_handler() -> Html<&'static str> {
    Html("Form submitted successfully!")
}
