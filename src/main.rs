mod my_api;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                    .route("/create", web::get().to(my_api::create))
                    .route("/read", web::get().to(my_api::read))
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}