use axum::Router;

mod schema;

#[tokio::main]
async fn main() {
    let service =
        schema::Chat::new().get_message_by_id(|req: schema::ChatGetMessageByIdRequest| async move {
            schema::Message {
                id: req.id,
                text: format!("Hello, {}", req.id),
            }
        });

    let router: Router = service.into();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:14514")
        .await
        .unwrap();

    println!("Listening on: {}", listener.local_addr().unwrap());

    axum::serve(listener, router).await.unwrap();
}
