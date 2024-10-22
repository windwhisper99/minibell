use actix_web::{
    http::StatusCode,
    web::{get, post, Data, Json, Query, ServiceConfig},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};

use crate::{
    api::dto::MemberDto,
    domain::{auth::AccessType, Error},
    infra,
    usecase::{get_auth_information, sign_in},
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SignInBody {
    code: String,
    redirect_uri: String,
}

async fn sign_in(
    body: Json<SignInBody>,
    member_repo: Data<infra::MemberRepo>,
    discord_req: Data<infra::DiscordReq>,
    session_repo: Data<infra::SessionRepo>,
    session_hmac: Data<infra::SessionHmac>,
) -> Result<impl Responder, Error> {
    let token = sign_in::SignInUC::new(
        member_repo.as_ref(),
        session_repo.as_ref(),
        session_hmac.as_ref(),
        discord_req.as_ref(),
    )
    .execute(&body.code, &body.redirect_uri)
    .await?;

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Result {
        token: String,
    }

    Ok(Json(Result { token }))
}

#[derive(Debug, Deserialize)]
struct GetAuthInformationQuery {
    redirect_uri: String,
}

async fn get_auth_information(
    query: Query<GetAuthInformationQuery>,
    access_type: AccessType,
    discord_req: Data<infra::DiscordReq>,
) -> Result<impl Responder, Error> {
    let result = get_auth_information::GetAuthInformation::new(discord_req.as_ref())
        .execute(
            get_auth_information::GetAuthInformationInput {
                redirect_uri: query.redirect_uri.clone(),
            },
            &access_type,
        )
        .await?;

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Result {
        discord_oauth_url: String,
        member: Option<MemberDto>,
    }

    Ok(HttpResponse::build(StatusCode::OK).json(Result {
        discord_oauth_url: result.discord_oauth_url,
        member: result.member.map(Into::into),
    }))
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.route("auth", get().to(get_auth_information))
        .route("auth", post().to(sign_in));
}
