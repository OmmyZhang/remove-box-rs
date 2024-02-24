use actix_cors::Cors;
use actix_web::web::{scope, Data, ServiceConfig};
use ring::hmac;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use shuttle_secrets::SecretStore;
use sqlx::{Executor, PgPool};

mod record;
mod sig;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    pool.execute(include_str!("../schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let key_value = secret_store.get("KEY_VALUE").unwrap();
    let key = hmac::Key::new(hmac::HMAC_SHA256, key_value.as_bytes());

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
                .app_data(Data::new(key))
                .app_data(Data::new(pool)),
        );
    };

    Ok(config.into())
}
