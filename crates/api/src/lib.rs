use std::sync::Arc;

use axum::{
    extract::{Query, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
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
    State(infra): State<Arc<InfraModule>>,
    Query(query): Query<GetAuthInfoQuery>,
) -> impl IntoResponse {
    use usecases::get_auth_info::*;

    let get_auth_info = GetAuthInfo {
        discord_client: infra.as_ref().resolve_ref(),
    };
    let auth_info = get_auth_info
        .execute(
            &AccessType::Guest,
            GetAuthInfoInput {
                redirect_uri: query.redirect_uri,
            },
        )
        .await
        .unwrap();

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Response {
        auth_url: String,
    }

    Json(Response {
        auth_url: auth_info.auth_url,
    })
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SignInJson {
    code: String,
    redirect_uri: String,
}

async fn sign_in(
    State(infra): State<Arc<InfraModule>>,
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

pub async fn app(config: infra::BootstrapConfig) -> Router {
    let infra = infra::bootstrap(config)
        .await
        .expect("Failed to bootstrap infra");

    let infra = Arc::new(infra);
    Router::new()
        .route("/", get(root))
        .route("/auth", get(get_auth_info))
        .route("/auth", post(sign_in))
        .with_state(infra)
}
