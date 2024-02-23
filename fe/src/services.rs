use gloo_net::http::Request;
use once_cell::sync::Lazy;
use ring::hmac;

use sig::{get_current_t, hmac_sign};

use crate::types::Record;

const API_BASE: &str = env!("API_BASE");
const KEY_VALUE: &str = env!("KEY_VALUE");
static KEY: Lazy<hmac::Key> = Lazy::new(|| hmac::Key::new(hmac::HMAC_SHA256, KEY_VALUE.as_bytes()));

type Result<T> = std::result::Result<T, String>;

pub async fn get_record_list() -> Result<Vec<Record>> {
    let t = get_current_t();
    let sig = hmac_sign(t, "list", &KEY);

    let r = Request::get(&format!("{}/record/list", API_BASE))
        .header("sig-t", &t.to_string())
        .header("sig", &sig)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if r.ok() {
        r.json().await.map_err(|e| e.to_string())
    } else {
        Err(r.status_text())
    }
}

pub async fn upload_record(record: Record) -> Result<Vec<Record>> {
    let t = get_current_t();
    let sig = hmac_sign(t, &record, &KEY);

    let r = Request::post(&format!("{}/record/upload", API_BASE))
        .header("sig-t", &t.to_string())
        .header("sig", &sig)
        .json(&record)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if r.ok() {
        r.json().await.map_err(|e| e.to_string())
    } else {
        Err(r.status_text())
    }
}
