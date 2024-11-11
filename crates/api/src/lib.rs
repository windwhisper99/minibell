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

mod duty {
    use std::sync::Arc;

    use axum::{
        extract::{Path, Query},
        response::IntoResponse,
        Extension, Json,
    };
    use infra::InfraModule;
    use minibell::{
        duty,
        usecases::{self, UseCase},
        AccessType,
    };
    use serde::{Deserialize, Serialize};
    use shaku::HasComponent;

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    struct DutyCategory {
        id: String,
        name: String,
        has_children: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        parent: Option<String>,
        sort: i32,
    }

    impl From<duty::DutyCategory> for DutyCategory {
        fn from(category: duty::DutyCategory) -> Self {
            Self {
                id: category.id,
                name: category.name,
                has_children: category.has_children,
                parent: category.parent,
                sort: category.sort,
            }
        }
    }

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Duty {
        id: String,
        category: String,
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        short_name: Option<String>,
        patch: String,
        image: String,
        sort: i32,
        #[serde(skip_serializing_if = "Option::is_none")]
        phrases: Option<Vec<DutyPhrase>>,
    }

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    struct DutyPhrase {
        name: String,
        progression: f64,
    }

    impl From<duty::Duty> for Duty {
        fn from(duty: duty::Duty) -> Self {
            Self {
                id: duty.id,
                category: duty.category,
                name: duty.name,
                description: duty.description,
                short_name: duty.short_name,
                patch: duty.patch.to_string(),
                image: duty.image,
                sort: duty.sort,
                phrases: None,
            }
        }
    }

    impl From<(duty::Duty, Vec<duty::DutyPhrase>)> for Duty {
        fn from((duty, phrases): (duty::Duty, Vec<duty::DutyPhrase>)) -> Self {
            Self {
                phrases: Some(
                    phrases
                        .into_iter()
                        .map(|phrase| DutyPhrase {
                            name: phrase.name,
                            progression: phrase.progression,
                        })
                        .collect::<Vec<_>>(),
                ),
                ..duty.into()
            }
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct GetDutiesQuery {
        category: String,
    }

    pub async fn get_duties(
        Extension(infra): Extension<Arc<InfraModule>>,
        Query(query): Query<GetDutiesQuery>,
    ) -> impl IntoResponse {
        use usecases::get_duties::*;

        let get_duties = GetDuties {
            duty_repo: infra.as_ref().resolve_ref(),
        };
        let duties = get_duties
            .execute(
                &AccessType::Guest,
                Input {
                    category: query.category,
                },
            )
            .await
            .expect("Failed to get duties");

        Json(
            duties
                .into_iter()
                .map(|duty| Duty::from(duty))
                .collect::<Vec<_>>(),
        )
    }

    #[derive(Debug, Deserialize)]
    pub struct GetCategoriesQuery {
        parent: Option<String>,
    }

    pub async fn get_categories(
        Query(query): Query<GetCategoriesQuery>,
        Extension(infra): Extension<Arc<InfraModule>>,
    ) -> impl IntoResponse {
        use usecases::get_duty_categories::*;

        let get_categories = GetDutyCategories {
            duty_repo: infra.as_ref().resolve_ref(),
        };
        let categories = get_categories
            .execute(
                &AccessType::Guest,
                Input {
                    parent: query.parent,
                },
            )
            .await
            .expect("Failed to get categories");

        Json(
            categories
                .into_iter()
                .map(|category| DutyCategory::from(category))
                .collect::<Vec<_>>(),
        )
    }

    pub async fn get_duty(
        Extension(infra): Extension<Arc<InfraModule>>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        use usecases::get_duty::*;

        let get_duty = GetDuty {
            duty_repo: infra.as_ref().resolve_ref(),
        };
        let (duty, phrases) = get_duty
            .execute(&AccessType::Guest, &id)
            .await
            .expect("Failed to get duty");

        Json(Duty::from((duty, phrases)))
    }
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
        .route("/duty_categories", get(duty::get_categories))
        .route("/duties", get(duty::get_duties))
        .route("/duties/:duty_id", get(duty::get_duty))
        .layer(Extension(infra))
}
