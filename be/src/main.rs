use actix_web::{web, App, HttpServer};

mod record;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api/record")
                .service(record::upload)
                .service(record::list),
        )
    })
    .bind(("127.0.0.1", 9876))?
    .run()
    .await
}
