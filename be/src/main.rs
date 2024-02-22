use actix_web::web::{scope, ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;

mod record;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            scope("/api/record")
                .service(record::upload)
                .service(record::list),
        );
    };

    Ok(config.into())
}
