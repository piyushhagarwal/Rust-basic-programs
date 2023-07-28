#[tokio::main]
async fn main() {

    //This is a simple get request
    let result = reqwest::get("https://api.spotify.com/v1/search").await.unwrap().text().await.unwrap();
    //.text gives the actual body of the response

    println!("{:#?}",result);//This syntax helps to prettify the output

    //Creating a get request using Client and adding headers to it

    let client = reqwest::Client::new();

    let response = client
        .get("https://api.spotify.com/v1/search")
        .header("AUTHORIZATION", "Bearer [AUTH_TOKEN]")
        .header("CONTENT_TYPE", "application/json")
        .header("ACCEPT", "application/json")
        .send()
        .await.
        unwrap().
        text().
        await;

    println!("{:#?}",response)

}
