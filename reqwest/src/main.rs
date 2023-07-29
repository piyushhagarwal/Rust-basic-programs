use serde::Deserialize;


#[derive(Debug,Deserialize)]
struct Source{
    id: Option<String>,
    name: String,
}
#[derive(Debug,Deserialize)]
struct Article{
    source: Source,
    author: Option<String>,
    title: String,
    description: String,
    url : String,
    urlToImage: Option<String>,
    publishedAt: String,
    content: String
}
#[derive(Debug, Deserialize)]
struct Response{
    status: String,
    totalResults: u32,
    articles: Vec<Article>
}


#[tokio::main]
async fn main() {

    let api_key = "7d20478f4cc6481eb401e66d7d5ca7b6";
    //Creating a get request using Client and adding headers to it
  
    let url = format!("https://newsapi.org/v2/everything?q=tesla&sortBy=publishedAt&apiKey={}", api_key);


    let client = reqwest::Client::new();

    let json_str = client
        .get(url)
        .send()
        .await.
        unwrap()
        .text()
        .await
        .unwrap();

    let response : Response = serde_json::from_str(&json_str).expect("Failed to deserialize JSON");


    println!("{:#?}",response)

}
