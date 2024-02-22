use actix_web::{get, post, web};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    name: String,
    score: u32,
}

#[post("/upload")]
pub async fn upload(record: web::Json<Record>) -> web::Json<Vec<Record>> {
    dbg!(&record);
    web::Json(vec![record.into_inner()])
}

#[get("/list")]
pub async fn list() -> web::Json<Vec<Record>> {
    web::Json(vec![])
}
