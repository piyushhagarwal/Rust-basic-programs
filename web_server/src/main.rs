//Installed "cargo-watch" crate to rerun the server whenever a change is made.
//It is similat to nodemon in javascript
//command to run the server: "cargo watch -x run"


use axum::{Router, routing::{get,post}, Json};
use axum::extract::{Path, Query};
use std::net::SocketAddr;
use serde::{Serialize,Deserialize};


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/hello",get(handler_function))
        .route("/test/:id", get(handle_id)) //syntax for handling dynamic routes
        .route("/:user_id/team/:team_id",get(handle_multiple_ids)) //Syntax for handling 2 or more dynamic parameters
        .route("/queryparams", get(handle_query))
        .route("/post", post(handle_post));


    //Code to start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
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

    //This function returns a string which holds the value of dynamic id.
    async fn handle_id(Path(id): Path<String>) -> String {
        format!("{id}")
    }

    //This function returns a string which holds the value of dynamic user_id. and team_id
    async fn handle_multiple_ids(Path((user_id,team_id)): Path<(u32,String)>) -> String {
        format!("user_id: {}, team_id: {}",user_id,team_id )
    }

    #[derive(Serialize,Deserialize)]
    struct QueryParams{
        message : String,
        id : i32
    }

    //This function returns a json of QueryParams type
    async fn handle_query(Query(query): Query<QueryParams>) -> Json<QueryParams>{
        Json(query)
    }

    #[derive(Serialize,Deserialize)]
    struct UserData {
        name: String,
        age: u32,
    }

    //This function gets the body of the api as an argument and returns a json of UserData type
    async fn handle_post(user_data: Json<UserData>) -> Json<UserData> {
        Json(UserData { name: user_data.name.to_string(), age: user_data.age })
    }
}



