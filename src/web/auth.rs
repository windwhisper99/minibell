use actix_web::{
    cookie::Cookie,
    web::{get, scope, Data, Query, Redirect, ServiceConfig},
    Responder,
};
use serde::Deserialize;

use crate::{infra, usecase::sign_in::SignInUC};

#[derive(Deserialize)]
struct RedirectQuery {
    code: String,
}

async fn redirect(
    query: Query<RedirectQuery>,
    member_repo: Data<infra::MemberRepo>,
    session_repo: Data<infra::SessionRepo>,
    session_hmac: Data<infra::SessionHmac>,
    discord_req: Data<infra::DiscordReq>,
) -> impl Responder {
    let token = match SignInUC::new(
        member_repo.as_ref(),
        session_repo.as_ref(),
        session_hmac.as_ref(),
        discord_req.as_ref(),
    )
    .execute(&query.code)
    .await
    {
        Ok(token) => token,
        Err(err) => {
            println!("{:?}", err);
            return Redirect::to("/").temporary().customize();
        }
    };

    let cookie = Cookie::build("token", token)
        // .same_site(SameSite::Strict)
        .http_only(true)
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
