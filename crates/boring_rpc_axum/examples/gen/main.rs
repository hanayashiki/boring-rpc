use std::fmt::format;

use axum::{
    extract::{Path, State},
    routing::any_service,
    Router,
};

mod gen;

#[tokio::main]
async fn main() {
    let service = gen::ExampleService::new()
        .method1(
            |State(state): State<&'static str>, req: gen::Method1Request| async move {
                gen::Method1Response {
                    answer: req.question.contains(state),
                }
            },
        )
        .method2(
            |Path(answer): Path<String>, req: gen::Method2Request| async {
                gen::Method2Response {}
            },
        )
        .method2(
            |Path(answer): Path<String>, req: gen::Method2Request| async {
                gen::Method2Response {}
            },
        )
        .with_state("42");

    let router: Router = service.into();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:14514")
        .await
        .unwrap();

    println!("Listening on: {}", listener.local_addr().unwrap());

    axum::serve(listener, router).await.unwrap();
}
