use serde::{Deserialize, Serialize};
use actix_web::{
    get,
    web,
    HttpResponse,
    client,
};

#[derive(Serialize, Deserialize)]
pub struct Groot{
    text: String
}

#[get("/api/groot")]
pub async fn groot(web::Query(_params): web::Query<Groot>) -> HttpResponse {
    println!("Params received {:?}",_params.text);
    let client = client::Client::default();
    let req = client.get("https://jsonplaceholder.typicode.com/todos/1")
        .header("User-Agent", "Actix-web")
        .send()
        .await
        .unwrap()
        .body()
        .await;
    println!("Response: {:?}", req);
    match req {
        Ok(payload) => {
            println!("Ok. {:?}", payload);
            return HttpResponse::Ok().content_type("application/json").body(payload);
        },
        Err(err) => {
            println!("Err. {:?}", err);
            return HttpResponse::Ok().content_type("text/html").body("payload");
        },
    }
}