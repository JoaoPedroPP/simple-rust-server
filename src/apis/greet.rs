use serde::{Deserialize, Serialize};
use serde_json;
use actix_web::{
    get,
    HttpResponse,
    client
};

#[derive(Serialize, Deserialize)]
pub struct Groot{
    text: String
}

#[derive(Serialize, Deserialize)]
pub struct API{
    userId: u32,
    id: u32,
    title: String,
    completed: bool
}

#[get("/api/greet")]
pub async fn greet() -> HttpResponse {
    let client = client::Client::default();
    let req = client.get("https://jsonplaceholder.typicode.com/todos/1")
        .header("User-Agent", "Actix-web")
        .send()
        .await
        .unwrap()
        .body()
        .await;
    // println!("{:?}", req);
    match req {
        Ok(payload) => {
            println!("Ok. {:?}", payload);
            let x: API = serde_json::from_slice(&payload).unwrap(); // Preciso obrigatoriamente definir o tipo
            return HttpResponse::Ok().json(x);
        },
        Err(err) => {
            println!("Err. {:?}", err);
            return HttpResponse::Ok().content_type("text/html").body("payload");
        },
    }
}