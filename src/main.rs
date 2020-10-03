// use serde::{Deserialize, Serialize};
// use serde_json::{Result, Value};
use actix_web::{
    get,
    web,
    App,
    HttpRequest,
    HttpServer,
    HttpResponse,
    Responder,
    client,
    HttpMessage
};

mod apis;

// #[derive(Serialize, Deserialize)]
// pub struct Groot{
//     text: String
// }

// #[derive(Serialize, Deserialize)]
// pub struct API{
//     userId: u32,
//     id: u32,
//     title: String,
//     completed: bool
// }

// #[get("/api/groot")]
// async fn groot(web::Query(params): web::Query<Groot>) -> HttpResponse {
//     let client = client::Client::default();
//     let req = client.get("https://jsonplaceholder.typicode.com/todos/1")
//         .header("User-Agent", "Actix-web")
//         .send()
//         .await
//         .unwrap()
//         .body()
//         .await;
//     println!("Response: {:?}", req);
//     match req {
//         Ok(payload) => {
//             println!("Ok. {:?}", payload);
//             return HttpResponse::Ok().content_type("application/json").body(payload);
//         },
//         Err(err) => {
//             println!("Err. {:?}", err);
//             return HttpResponse::Ok().content_type("text/html").body("payload");
//         },
//     }
// }

// #[get("/api/greet")]
// async fn greet(web::Query(params): web::Query<Groot>) -> HttpResponse {
//     let client = client::Client::default();
//     let req = client.get("https://jsonplaceholder.typicode.com/todos/1")
//         .header("User-Agent", "Actix-web")
//         .send()
//         .await
//         .unwrap()
//         .body()
//         .await;
//     // println!("{:?}", req);
//     match req {
//         Ok(payload) => {
//             println!("Ok. {:?}", payload);
//             let x: API = serde_json::from_slice(&payload).unwrap(); // Preciso obrigatoriamente definir o tipo
//             return HttpResponse::Ok().json(x);
//         },
//         Err(err) => {
//             println!("Err. {:?}", err);
//             return HttpResponse::Ok().content_type("text/html").body("payload");
//         },
//     }
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(apis::groot)
            .service(apis::greet)
            // .route("/", web::get().to(groot))
    })
    .bind("0.0.0.0:8000")?
    .workers(4)
    .run()
    .await
}