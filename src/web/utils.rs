use std::{future::Future, pin::Pin};

use actix_web::{
    body::BoxBody,
    http::{
        header::{ContentType, HeaderName, HeaderValue, InvalidHeaderValue, TryIntoHeaderPair},
        StatusCode,
    },
    web::Data,
    FromRequest, HttpResponse,
};
use askama::Template;

use crate::{
    domain::{auth::AccessType, Error},
    infra,
    usecase::verify_auth::VerifyAuthUC,
};

pub struct HxLocation(pub &'static str);

impl TryIntoHeaderPair for HxLocation {
    type Error = InvalidHeaderValue;

    fn try_into_pair(self) -> Result<(HeaderName, HeaderValue), Self::Error> {
        Ok((
            HeaderName::from_static("hx-location"),
            HeaderValue::from_str(&self.0)?,
        ))
    }
}

/// Extractor for AccessType
impl FromRequest for AccessType {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let token = req.cookie("token").map(|cookie| cookie.value().to_string());
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

        #[derive(Template)]
        #[template(path = "unauthorized.html")]
        struct UnauthorizedTempl {}

        match self {
            Error::Unauthenticated => HttpResponse::build(self.status_code())
                .content_type(ContentType::html())
                .body(
                    UnauthorizedTempl {}
                        .render()
                        .unwrap_or("Internal Server Error".to_string()),
                ),
            _ => HttpResponse::build(self.status_code())
                .content_type(ContentType::html())
                .body(
                    UnauthorizedTempl {}
                        .render()
                        .unwrap_or("Internal Server Error".to_string()),
                ),
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

pub fn authorizated_check(access_type: &AccessType) -> Result<(), Error> {
    match access_type {
        AccessType::Session(_) => Ok(()),
        _ => Err(Error::Unauthenticated),
    }
}

pub mod templates {
    use askama::Template;

    use crate::{
        domain::{
            auth::AccessType,
            member::{DiscordService, Member},
        },
        infra::DiscordReq,
    };

    enum UserStatus {
        Unauth,
        Auth(Member),
    }

    impl From<&AccessType> for UserStatus {
        fn from(access_type: &AccessType) -> Self {
            match access_type {
                AccessType::Session(session_with_member) => {
                    UserStatus::Auth(session_with_member.member.clone())
                }
                _ => UserStatus::Unauth,
            }
        }
    }

    #[derive(Template)]
    #[template(path = "components/user_status.html")]
    pub struct UserStatusTempl {
        status: UserStatus,
        auth_url: String,
    }

    impl UserStatusTempl {
        pub fn new(access_type: &AccessType, discord_req: &DiscordReq) -> Self {
            Self {
                status: UserStatus::from(access_type),
                auth_url: discord_req.get_oauth2_url(),
            }
        }
    }
}
