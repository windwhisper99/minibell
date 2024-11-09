use std::sync::Arc;

use axum::{
    async_trait,
    extract::{FromRequest, Query, Request},
    http::{header, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use infra::InfraModule;
use minibell::{
    usecases::{self, UseCase},
    AccessType,
};
use serde::{Deserialize, Serialize};
use shaku::HasComponent;

async fn root() -> impl IntoResponse {
    #[derive(Debug, Clone, Serialize)]
    struct Message {
        message: String,
    }

    Json(Message {
        message: "Root message!".to_string(),
    })
}

#[derive(Debug, Deserialize)]
struct GetAuthInfoQuery {
    redirect_uri: String,
}

async fn get_auth_info(
    Extension(infra): Extension<Arc<InfraModule>>,
    Query(query): Query<GetAuthInfoQuery>,
    AccessTypeHeader(access_type): AccessTypeHeader,
) -> impl IntoResponse {
    use usecases::get_auth_info::*;

    let get_auth_info = GetAuthInfo {
        discord_client: infra.as_ref().resolve_ref(),
        member_repo: infra.as_ref().resolve_ref(),
    };
    let auth_info = get_auth_info
        .execute(
            &access_type,
            GetAuthInfoInput {
                redirect_uri: query.redirect_uri,
            },
        )
        .await
        .unwrap();

    #[derive(Debug, Serialize)]
    struct Member {
        id: u64,
        name: String,
        avatar: String,
    }

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Response {
        auth_url: String,
        member: Option<Member>,
    }

    Json(Response {
        auth_url: auth_info.auth_url,
        member: auth_info.member.map(|member| Member {
            id: member.id,
            name: member.display_name,
            avatar: member.avatar,
        }),
    })
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SignInJson {
    code: String,
    redirect_uri: String,
}

async fn sign_in(
    Extension(infra): Extension<Arc<InfraModule>>,
    Json(json): Json<SignInJson>,
) -> impl IntoResponse {
    use usecases::sign_in::*;

    let sign_in = SignIn {
        discord_client: infra.as_ref().resolve_ref(),
        member_repo: infra.as_ref().resolve_ref(),
        member_session_signer: infra.as_ref().resolve_ref(),
    };
    let token = sign_in
        .execute(
            &AccessType::Guest,
            SignInInput {
                discord_code: json.code,
                redirect_uri: json.redirect_uri,
            },
        )
        .await
        .unwrap();

    #[derive(Debug, Serialize)]
    struct Response {
        token: String,
    }

    Json(Response { token })
}

#[derive(Debug)]
struct AccessTypeHeader(AccessType);
#[async_trait]
impl<S> FromRequest<S> for AccessTypeHeader
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request(req: Request, _: &S) -> Result<Self, Self::Rejection> {
        let auth_header = req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok());

        if let Some(header) = auth_header {
            use usecases::authorization::*;

            let token = header
                .split("Bearer ")
                .last()
                .ok_or(StatusCode::UNAUTHORIZED)?;
            let infra = req
                .extensions()
                .get::<Arc<InfraModule>>()
                .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

            let authorization = Authorization {
                member_repo: infra.resolve_ref(),
                member_session_signer: infra.resolve_ref(),
            };

            match authorization
                .execute(&AccessType::Guest, token)
                .await
                .map_err(|_| StatusCode::UNAUTHORIZED)?
            {
                Some(session) => Ok(AccessTypeHeader(AccessType::Member(session.member_id))),
                None => Ok(AccessTypeHeader(AccessType::Guest)),
            }
        } else {
            Ok(AccessTypeHeader(AccessType::Guest))
        }
    }
}

pub async fn app(config: infra::BootstrapConfig) -> Router {
    let infra = infra::bootstrap(config)
        .await
        .expect("Failed to bootstrap infra");

    let infra = Arc::new(infra);
    Router::new()
        .route("/", get(root))
        .route("/auth", get(get_auth_info))
        .route("/auth", post(sign_in))
        .layer(Extension(infra))
}
