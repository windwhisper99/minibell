use actix_web::{
    cookie::Cookie,
    http::StatusCode,
    web::{get, scope, Data, Query, Redirect, ServiceConfig},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};

use crate::{
    domain::{auth::AccessType, Error},
    infra,
    usecase::{get_auth_information, sign_in},
};

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
    let token = match sign_in::SignInUC::new(
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

#[derive(Debug, Deserialize)]
struct GetAuthInformationInput {
    redirect_uri: String,
}

async fn get_auth_information(
    body: Query<GetAuthInformationInput>,
    access_type: AccessType,
    discord_req: Data<infra::DiscordReq>,
) -> Result<impl Responder, Error> {
    let result = get_auth_information::GetAuthInformation::new(discord_req.as_ref())
        .execute(
            get_auth_information::GetAuthInformationInput {
                redirect_uri: body.redirect_uri.clone(),
            },
            &access_type,
        )
        .await?;
    println!("{:?}", result);

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Result {
        discord_oauth_url: String,
    }

    Ok(HttpResponse::build(StatusCode::OK).json(Result {
        discord_oauth_url: result.discord_oauth_url,
    }))
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("auth").route("/redirect", get().to(redirect)))
        .route("api/auth", get().to(get_auth_information));
}
