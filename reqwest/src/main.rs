#[tokio::main]
async fn main() {
    let result = reqwest::get("https://api.spotify.com/v1/search").await.expect("Error in fetching the api");
    //This is a simple get request

    println!("{:#?}",result);//This syntax helps to prettify the output
}
