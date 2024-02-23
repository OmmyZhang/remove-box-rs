use actix_cors::Cors;
use actix_web::web::{scope, Data, ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::{Executor, PgPool};

mod record;
mod sig;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {

    #[cfg(debug_assertions)]
    pool.execute(include_str!("../schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let config = move |cfg: &mut ServiceConfig| {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(["GET", "POST"])
            .allowed_headers(["content-type", "sig", "sig-t"]);
        cfg.service(
            scope("/api/record")
                .wrap(cors)
                .service(record::upload)
                .service(record::list)
                .app_data(Data::new(pool)),
        );
    };

    Ok(config.into())
}
