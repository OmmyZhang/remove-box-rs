use actix_web::{
    error::ErrorBadRequest,
    get, post,
    web::{Data, Json},
    Result,
};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::{FromRow, PgPool};

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Record {
    name: String,
    score: i32,
    time: Option<NaiveDateTime>,
}

#[post("/upload")]
pub async fn upload(record: Json<Record>, pool: Data<PgPool>) -> Result<Json<Vec<Record>>> {
    sqlx::query(
        "INSERT INTO records(name, score) VALUES ($1, $2)
            ON CONFLICT (name)
                DO UPDATE SET score=$2, time = CURRENT_TIMESTAMP
                WHERE $2 > records.score",
    )
    .bind(record.name.clone())
    .bind(record.score)
    .execute(pool.as_ref())
    .await
    .map_err(|e| ErrorBadRequest(e.to_string()))?;

    query_records_by_score(&pool).await.map(Json)
}

async fn query_records_by_score(pool: &PgPool) -> Result<Vec<Record>> {
    sqlx::query_as("SELECT name, score, time FROM records ORDER BY score DESC, time ASC")
        .fetch_all(pool)
        .await
        .map_err(|e| ErrorBadRequest(e.to_string()))
}

#[get("/list")]
pub async fn list(pool: Data<PgPool>) -> Result<Json<Vec<Record>>> {
    query_records_by_score(&pool).await.map(Json)
}
