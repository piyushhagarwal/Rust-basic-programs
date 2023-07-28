
struct Source<T>{
    id: Option<T>,
    name: String,
}
struct Article{
    source: Source<i32>,
    name: String,
    author: String,
    title: String,
    description: String

}


#[tokio::main]
async fn main() {

    let api_key = "7d20478f4cc6481eb401e66d7d5ca7b6";
    //Creating a get request using Client and adding headers to it
  
    let url = format!("https://newsapi.org/v2/everything?q=tesla&from=2023-06-28&sortBy=publishedAt&apiKey={api_key}");

    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .send()
        .await.
        unwrap()
        .text()
        .await
        .unwrap();

    println!("{:#?}",response)

}
