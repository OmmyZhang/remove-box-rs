use sig::get_current_t;

use actix_web::{dev::Payload, error::ErrorUnauthorized, Error, FromRequest, HttpRequest};
use futures::future;

#[derive(Debug)]
pub struct SigInfo {
    pub t: u64,
    pub sig: String,
}

impl FromRequest for SigInfo {
    type Error = Error;
    type Future = future::Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let curr_t = get_current_t();
        let headers = req.headers();

        headers
            .get("sig-t")
            .and_then(|t| t.to_str().ok())
            .and_then(|t| t.parse::<u64>().ok())
            .and_then(|t| (t.abs_diff(curr_t) < 10).then_some(t))
            .and_then(|t| {
                headers
                    .get("sig")
                    .and_then(|sig| sig.to_str().ok())
                    .map(|sig| {
                        future::ok(SigInfo {
                            t,
                            sig: sig.to_string(),
                        })
                    })
            })
            .unwrap_or(future::err(ErrorUnauthorized("no acceptable signature")))
    }
}
