use gloo_net::http::Request;

use crate::types::Record;

const API_BASE: &str = "http://127.0.0.1:8000/api";
type Result<T> = std::result::Result<T, String>;

pub async fn get_record_list() -> Result<Vec<Record>> {
    Request::get(&format!("{}/record/list", API_BASE))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}
