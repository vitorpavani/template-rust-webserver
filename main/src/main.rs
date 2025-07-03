use axum::{
    Router,
    routing::{get, post},
};

mod handlers;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(handlers::handler))
        .route("/submit", post(handlers::submit_handler));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to address");

    println!(
        "listening on {}",
        listener.local_addr().expect("Failed to get local address")
    );
    axum::serve(listener, app)
        .await
        .expect("Failed to run server");
}
