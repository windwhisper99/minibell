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
struct DutyCategoryDto {
    id: String,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<String>,
}

impl From<duty::DutyCategory> for DutyCategoryDto {
    fn from(category: duty::DutyCategory) -> Self {
        Self {
            id: category.id,
            name: category.name,
            parent: category.parent,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct DutyDto {
    id: String,
    category: String,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    short_name: Option<String>,
    patch: String,
    image: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    phrases: Option<Vec<DutyPhraseDto>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct DutyPhraseDto {
    name: String,
    progression: f64,
}

impl From<duty::Duty> for DutyDto {
    fn from(duty: duty::Duty) -> Self {
        Self {
            id: duty.id,
            category: duty.category,
            name: duty.name,
            description: duty.description,
            short_name: duty.short_name,
            patch: duty.patch.to_string(),
            image: duty.image,
            phrases: None,
        }
    }
}

impl From<(duty::Duty, Vec<duty::DutyPhrase>)> for DutyDto {
    fn from((duty, phrases): (duty::Duty, Vec<duty::DutyPhrase>)) -> Self {
        Self {
            phrases: Some(
                phrases
                    .into_iter()
                    .map(|phrase| DutyPhraseDto {
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
    category: Option<String>,
}

pub async fn get_duties(
    Extension(infra): Extension<Arc<InfraModule>>,
    Query(query): Query<GetDutiesQuery>,
) -> impl IntoResponse {
    use usecases::get_duties::*;

    let get_duties = GetDuties {
        duty_repo: infra.as_ref().resolve_ref(),
    };
    let response = get_duties
        .execute(
            &AccessType::Guest,
            Input {
                category: query.category,
            },
        )
        .await
        .expect("Failed to get duties");

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Response {
        breadcrumbs: Vec<DutyCategoryDto>,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        categories: Vec<DutyCategoryDto>,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        duties: Vec<DutyDto>,
    }

    Json(Response {
        breadcrumbs: response.breadcrumbs.into_iter().map(From::from).collect(),
        categories: response.categories.into_iter().map(From::from).collect(),
        duties: response.duties.into_iter().map(From::from).collect(),
    })
}

pub async fn get_duty(
    Extension(infra): Extension<Arc<InfraModule>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    use usecases::get_duty::*;

    let get_duty = GetDuty {
        duty_repo: infra.as_ref().resolve_ref(),
    };
    let response = get_duty
        .execute(&AccessType::Guest, &id)
        .await
        .expect("Failed to get duty");

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Response {
        breadcrumbs: Vec<DutyCategoryDto>,
        duty: DutyDto,
    }

    Json(Response {
        breadcrumbs: response.breadcrumbs.into_iter().map(From::from).collect(),
        duty: (response.duty, response.phrases).into(),
    })
}
