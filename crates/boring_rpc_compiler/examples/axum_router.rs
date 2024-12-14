use axum::{extract::State, routing::get, Router};

#[derive(Debug, Clone)]
struct S1 {
    fuck: String,
}

struct NotClone {}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/a", get(|| async { "" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:14514")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
