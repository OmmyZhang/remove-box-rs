use std::fmt::Write;

#[cfg(target_arch = "wasm32")]
use js_sys::Date;
use ring::hmac;
use serde::ser::Serialize;
#[cfg(not(target_arch = "wasm32"))]
use std::time::SystemTime;

pub fn hmac_sign<T>(t: u64, content: &T, key: &hmac::Key) -> String
where
    T: ?Sized + Serialize,
{
    let content_str = serde_json::to_string(content).unwrap();
    hex_encode(hmac::sign(key, format!("[{}|{}]", t, content_str).as_bytes()).as_ref())
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().fold(String::new(), |mut output, b| {
        let _ = write!(output, "{b:02x}");
        output
    })
}

#[cfg(target_arch = "wasm32")]
pub fn get_current_t() -> u64 {
    Date::now() as u64 / 1000
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_current_t() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
