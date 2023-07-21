//Installed "cargo-watch" crate to rerun the server whenever a change is made.
//It is similat to nodemon in javascript
//command to run the server: "cargo watch -x run"


use axum::{Router, routing::get, Json};
use std::net::SocketAddr;
use serde::Serialize;


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


    #[derive(Serialize)]
    struct Message {
        message: String,
    }

    //This function returns a json object as a response for this function.
    async fn handler_function () -> Json<Message>{
        Json(Message { 
            message: String::from("Hello, world. I am test") 
        })
        //This Json function serializes(convert struct into json) the Message struct.
    }
}



