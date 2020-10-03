use actix_web::{
    App,
    HttpServer
};

mod apis;

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