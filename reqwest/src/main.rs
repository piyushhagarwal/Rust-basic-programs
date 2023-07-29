use serde::Deserialize;

//Created different structs for switching the json string of the response to a Struct
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
  
    let url = format!("https://newsapi.org/v2/everything?q=tesla&sortBy=publishedAt&apiKey={}", api_key);


    let client = reqwest::Client::new();

    let json_str = client
        .get(url)
        .send()    //Sends the request
        .await
        .unwrap()
        .text()  //Gives the actual body or response of the api
        .await
        .unwrap();

    let response : Response = serde_json::from_str(&json_str).expect("Failed to deserialize JSON"); 
    // Converts Json string into a valid struct on which we can perform various operations


    println!("{:#?}",response) //This is to beautify the response 

}


//This is the example response we get when we hit the api
//  r#"{
//     "status": "ok",
//     "totalResults": 15403,
//     "articles": [
//         {
//             "source": {
//                 "id": "focus",
//                 "name": "Focus"
//             },
//             "author": "Von Gastautor Gabor Steingart (Berlin)",
//             "title": "Gastbeitrag von Gabor Steingart - Europa ist hinter die USA zurückgefallen - das sind unsere Schwachstellen",
//             "description": "Die Börsianer in Europa sind weiter optimistisch, aber die Frühindikatoren drücken auf die Stimmung: Der Einkaufsmanagerindex für die Privatwirtschaft sank im Juli um 1,0 auf 48,9 Punkte. „Werte unter 50 deuten auf eine Schrumpfung hin“, sagt der Kapitalmarkt…",
//             "url": "https://www.focus.de/finanzen/boerse/aktien/gastbeitrag-von-gabor-steingart-europa-ist-hinter-die-usa-zurueckgefallen-das-sind-unsere-schwachstellen_id_200263317.html",
//             "urlToImage": "https://p6.focus.de/img/fotos/id_43664671/5fa0d200c9d31616.jpg?im=Crop%3D%280%2C50%2C1600%2C800%29%3BResize%3D%281200%2C627%29&impolicy=perceptual&quality=mediumHigh&hash=a3d9622a9e772446043d9aedfd6313beb5dc3b13893707159afd885df88a4164",
//             "publishedAt": "2023-07-28T07:38:07Z",
//             "content": "Die Börsianer in Europa sind weiter optimistisch, aber die Frühindikatoren drücken auf die Stimmung: Der Einkaufsmanagerindex für die Privatwirtschaft sank im Juli um 1,0 auf 48,9 Punkte. Werte unter… [+6447 chars]"
//         },
//         {
//             "source": {
//                 "id": null,
//                 "name": "MacGeneration"
//             },
//             "author": "Nicolas Furno",
//             "title": "L’autonomie affichée dans les Tesla est-elle trompeuse ?",
//             "description": "Tesla tricherait sur l’autonomie annoncée de ses voitures électriques et le constructeur aurait mis en place une équipe secrète chargée d’empêcher ses clients d’amener leur voiture dans ses centres de service pour se plaindre d’une autonomie réelle inférieure…",
//             "url": "https://www.macg.co/mobilites/2023/07/tesla-affiche-t-il-une-autonomie-trompeuse-dans-ses-voitures-138395",
//             "urlToImage": "https://cdn.mgig.fr/2023/07/mga-7d050da1-w375-w1500-w750_accroche.jpg",
//             "publishedAt": "2023-07-28T08:00:23Z",
//             "content": "Tesla tricherait sur lautonomie annoncée de ses voitures électriques et le constructeur aurait mis en place une équipe secrète chargée dempêcher ses clients damener leur voiture dans ses centres de s… [+2696 chars]"
//         }
//     ]
// }"#