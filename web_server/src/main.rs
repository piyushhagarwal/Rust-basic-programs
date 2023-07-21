use axum::{Router, routing::get, response::Html};
use std::net::SocketAddr;


#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/hello",get(handler_function));


    //Code to start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    async fn handler_function () -> Html<&'static str>{
        Html("<h1>Hello world</h1>")
    }
}



