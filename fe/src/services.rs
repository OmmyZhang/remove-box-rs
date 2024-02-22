use gloo_net::http::Request;
use ring::hmac;
use web_sys::js_sys::Date;

use crate::types::Record;

const API_BASE: &str = "http://127.0.0.1:8000/api";
const KEY_VALUE: &str = "develop-key";

type Result<T> = std::result::Result<T, String>;

pub async fn get_record_list() -> Result<Vec<Record>> {
    let current = Date::now().to_string();

    let key = hmac::Key::new(hmac::HMAC_SHA256, KEY_VALUE.as_bytes());
    let sig: String = hmac::sign(&key, current.as_bytes())
        .as_ref()
        .iter()
        .map(|v| format!("{:02x}", v))
        .collect();

    Request::get(&format!("{}/record/list", API_BASE))
        .header("Sig-T", &current)
        .header("Sig", &sig)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}
