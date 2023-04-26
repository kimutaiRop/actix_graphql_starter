use actix_web::{
    dev::Payload, http::header::HeaderValue, Error as ActixWebError, FromRequest, HttpRequest,
};

use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};

use crate::utils::get_user_id;

#[derive(Serialize, Deserialize, Clone)]
pub struct AuthenticationToken {
    pub id: Option<i32>,
    pub authenticated: bool,
}

impl FromRequest for AuthenticationToken {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = req.clone();
        let authorization_header_option: Option<&HeaderValue> =
            req.headers().get(actix_web::http::header::AUTHORIZATION);

        if authorization_header_option.is_none() {
            return ready(Ok(AuthenticationToken {
                id: None,
                authenticated: false,
            }));
        }

        let authentication_token: String = authorization_header_option
            .unwrap()
            .to_str()
            .unwrap_or("")
            .to_string();

        if authentication_token.is_empty() {
            return ready(Ok(AuthenticationToken {
                id: None,
                authenticated: false,
            }));
        }
        let authentication_token: Vec<&str> = authentication_token.split(" ").collect();
        let authentication_token = authentication_token[1];
        let token_result = get_user_id(&authentication_token);

        if token_result == 0 {
            return ready(Ok(AuthenticationToken {
                id: None,
                authenticated: false,
            }));
        }

        ready(Ok(AuthenticationToken {
            id: Some(token_result),
            authenticated: true,
        }))
    }
}
