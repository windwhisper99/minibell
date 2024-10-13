use actix_web::{
    cookie::{Cookie, SameSite},
    web::{get, scope, Data, Query, Redirect, ServiceConfig},
    Responder,
};
use serde::Deserialize;

use crate::services::auth::AuthService;

#[derive(Deserialize)]
struct RedirectQuery {
    code: String,
}

async fn redirect(query: Query<RedirectQuery>, auth_service: Data<AuthService>) -> impl Responder {
    let session = match auth_service.auth(&query.code).await {
        Ok(session) => session,
        Err(err) => {
            println!("{:?}", err);
            return Redirect::to("/").temporary().customize();
        }
    };
    let cookie = Cookie::build("token", session)
        .same_site(SameSite::Strict)
        .http_only(true)
        .secure(true)
        .path("/")
        .finish();

    Redirect::to("/")
        .temporary()
        .customize()
        .add_cookie(&cookie)
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("auth").route("/redirect", get().to(redirect)));
}
