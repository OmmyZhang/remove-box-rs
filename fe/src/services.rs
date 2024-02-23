use gloo_net::http::Request;
use once_cell::sync::Lazy;
use ring::hmac;
use web_sys::js_sys::Date;

use crate::types::Record;

const API_BASE: &str = "http://127.0.0.1:8000/api";
const KEY_VALUE: &str = "develop-key";
static KEY: Lazy<hmac::Key> = Lazy::new(|| hmac::Key::new(hmac::HMAC_SHA256, KEY_VALUE.as_bytes()));

type Result<T> = std::result::Result<T, String>;

pub async fn get_record_list() -> Result<Vec<Record>> {
    let t = Date::now().to_string();
    let sig: String = hmac::sign(&KEY, t.as_bytes())
        .as_ref()
        .iter()
        .map(|v| format!("{:02x}", v))
        .collect();

    let r = Request::get(&format!("{}/record/list", API_BASE))
        .header("Sig-T", &t)
        .header("Sig", &sig)
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
    let t = Date::now().to_string();
    let msg = format!("{}{}", &t, serde_json::to_string(&record).unwrap());
    gloo_console::log!(&msg);
    let sig: String = hmac::sign(&KEY, msg.as_bytes())
        .as_ref()
        .iter()
        .map(|v| format!("{:02x}", v))
        .collect();

    let r = Request::post(&format!("{}/record/upload", API_BASE))
        .header("Sig-T", &t)
        .header("Sig", &sig)
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
