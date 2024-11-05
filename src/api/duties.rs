use actix_web::{
    web::{get, scope, Data, Path, ServiceConfig},
    HttpResponse, Responder,
};
use serde::Deserialize;

use crate::{
    domain::Error,
    infra,
    usecase::{get_duties_list, get_duty},
};

use super::dto::{self, DutyDto};

async fn list(duty_repo: Data<infra::DutyRepo>) -> Result<impl Responder, Error> {
    let duties = get_duties_list::GetDutiesList::new(duty_repo.get_ref())
        .execute()
        .await?
        .into_iter()
        .map(dto::DutyCategoryWithDutiesReviewDto::from)
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(duties))
}

#[derive(Debug, Deserialize)]
struct GetDutyParams {
    id: String,
}

async fn get_duty(
    params: Path<GetDutyParams>,
    duty_repo: Data<infra::DutyRepo>,
) -> Result<impl Responder, Error> {
    let duty = get_duty::GetDuty::new(duty_repo.get_ref())
        .execute(&params.id)
        .await?;

    Ok(HttpResponse::Ok().json(DutyDto::from(duty)))
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("duties")
            .route("", get().to(list))
            .route("{id}", get().to(get_duty)),
    );
}
