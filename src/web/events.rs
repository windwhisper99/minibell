use actix_web::{
    web::{get, post, scope, Data, Json, ServiceConfig},
    HttpResponse, Responder,
};
use askama::Template;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

use crate::utils::{db::Database, header::HxLocation};

mod create {
    use std::sync::Arc;

    use serde_with::{serde_as, TimestampSeconds};

    use crate::{
        domain::{auth::AccessType, Error},
        infra::DiscordReq,
        web::utils::{authorizated_check, templates},
    };

    use super::*;

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "snake_case")]
    enum SubmitType {
        Create,
        Save,
    }

    #[serde_as]
    #[derive(Debug, Deserialize)]
    struct Input {
        submit_type: SubmitType,
        title: String,
        description: Option<String>,
        #[serde_as(as = "TimestampSeconds<i64>")]
        start_at: DateTime<Utc>,
        #[serde_as(as = "Option<TimestampSeconds<i64>>")]
        deadline_at: Option<DateTime<Utc>>,
        slots: Vec<SlotInput>,
    }

    #[derive(Debug, Deserialize)]
    struct SlotInput {
        jobs: Vec<String>,
    }

    async fn submit(
        access_type: AccessType,
        input: Json<Input>,
        db: Data<Arc<Database>>,
    ) -> Result<impl Responder, Error> {
        authorizated_check(&access_type)?;

        let timestamp = Utc::now().timestamp_millis() as u64;
        let id = sqids::Sqids::default().encode(&[timestamp]).unwrap();

        #[derive(Debug, Serialize)]
        struct Input {
            id: String,
            title: String,
            description: Option<String>,
            start_at: DateTime<Utc>,
            deadline_at: Option<DateTime<Utc>>,
            status: String,

            slots: Vec<Vec<RecordId>>,
        }

        let input = Input {
            id: id.clone(),
            title: input.title.clone(),
            description: input.description.clone(),
            start_at: input.start_at,
            deadline_at: input.deadline_at,
            status: match &input.submit_type {
                SubmitType::Create => "private".to_string(),
                SubmitType::Save => "draft".to_string(),
            },
            slots: input
                .slots
                .iter()
                .map(|slot| {
                    slot.jobs
                        .iter()
                        .map(|j| RecordId::from_table_key("job", j))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<Vec<_>>>(),
        };

        db.query(
            "CREATE ONLY event SET
                    id = $id,
                    title = $title,
                    description = $description,
                    status = $status,
                    start_at = <datetime>$start_at,
                    deadline_at = $deadline_at && <datetime>$deadline_at,
                    slots = $slots.map(|$c| {jobs: $c})",
        )
        .bind(input)
        .await
        .unwrap()
        .check()
        .unwrap();

        Ok(HttpResponse::Created()
            .append_header(HxLocation("/"))
            .finish())
    }

    async fn page(
        access_type: AccessType,
        discord_req: Data<DiscordReq>,
    ) -> Result<impl Responder, Error> {
        authorizated_check(&access_type)?;

        #[derive(Template)]
        #[template(path = "create_event.html", escape = "none")]
        struct CreateEventPage {
            user_status: templates::UserStatusTempl,
        }

        Ok(CreateEventPage {
            user_status: templates::UserStatusTempl::new(&access_type, &discord_req),
        })
    }

    pub fn config(cfg: &mut ServiceConfig) {
        cfg.service(
            scope("create")
                .route("", get().to(page))
                .route("", post().to(submit)),
        );
    }
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("events").configure(create::config));
}
