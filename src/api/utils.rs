use std::{future::Future, pin::Pin};

use actix_web::{body::BoxBody, http::StatusCode, web::Data, FromRequest, HttpResponse};

use crate::{
    domain::{auth::AccessType, Error},
    infra,
    usecase::verify_auth::VerifyAuthUC,
};

/// Extractor for AccessType
impl FromRequest for AccessType {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let token = req
            .headers()
            .get("Authorization")
            .map(|header| {
                header
                    .to_str()
                    .expect("Authorization header is not a valid string")
            })
            .map(|header| header.replace("Bearer ", ""));

        let session_hmac = req
            .app_data::<Data<infra::SessionHmac>>()
            .expect("SessionHmac not found")
            .clone();
        let session_repo = req
            .app_data::<Data<infra::SessionRepo>>()
            .expect("SessionRepo not found")
            .clone();

        Box::pin(async move {
            match token {
                Some(token) => VerifyAuthUC::new(session_repo.as_ref(), session_hmac.as_ref())
                    .execute(&token)
                    .await
                    .map_err(|_| actix_web::error::ErrorInternalServerError("Verify error")),
                _ => Ok(AccessType::Unauthenticated),
            }
        })
    }
}

impl actix_web::ResponseError for Error {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        println!("{:?}", self);

        match self {
            Error::Unauthenticated => HttpResponse::build(self.status_code()).finish(),
            _ => HttpResponse::build(self.status_code()).finish(),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Error::Unauthenticated => StatusCode::UNAUTHORIZED,

            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::Forbidden => StatusCode::FORBIDDEN,

            Error::NotFound => StatusCode::NOT_FOUND,

            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// pub fn authorizated_check(access_type: &AccessType) -> Result<(), Error> {
//     match access_type {
//         AccessType::Session(_) => Ok(()),
//         _ => Err(Error::Unauthenticated),
//     }
// }
//
