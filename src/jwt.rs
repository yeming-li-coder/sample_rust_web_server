use std::future::{ready, Ready};

use actix_web::{Error, FromRequest};

pub struct JwToken {
    pub message: String,
}

impl FromRequest for JwToken {
    type Error = Error;

    type Future = Ready<Result<JwToken, Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        ready(match req.headers().get("token") {
            Some(data) => Ok(JwToken {
                message: data.to_str().unwrap().to_string(),
            }),
            None => Ok(JwToken {
                message: "nothing found".to_string(),
            }),
        })
    }
}
