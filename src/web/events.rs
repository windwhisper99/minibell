use actix_web::{
    web::{get, post, scope, Data, Json, ServiceConfig},
    HttpResponse, Responder,
};
use askama::Template;
use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::utils::HxLocation;

mod create {

    use chrono::Duration;
    use serde_with::{serde_as, TimestampSeconds};

    use crate::{
        domain::{auth::AccessType, event, Error},
        infra,
        usecase::create_event,
        web::utils::{authorizated_check, templates},
    };

    use super::*;

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "snake_case")]
    enum SubmitType {
        Publish,
        Save,
    }

    #[serde_as]
    #[derive(Debug, Deserialize)]
    struct Input {
        submit: SubmitType,
        title: String,
        description: Option<String>,

        // Slots
        slots: Vec<SlotInput>,

        // Schedule
        #[serde_as(as = "TimestampSeconds<i64>")]
        start_at: DateTime<Utc>,
        #[serde_as(as = "Option<TimestampSeconds<i64>>")]
        deadline_at: Option<DateTime<Utc>>,
    }

    #[derive(Debug, Deserialize)]
    struct SlotInput {
        jobs: Vec<String>,
    }

    async fn submit(
        access_type: AccessType,
        input: Json<Input>,
        event_repo: Data<infra::EventRepo>,
    ) -> Result<impl Responder, Error> {
        authorizated_check(&access_type)?;

        let event = create_event::CreateEventUC::new(event_repo.as_ref())
            .execute(
                &access_type,
                create_event::CreateEventUCInput {
                    kind: match &input.submit {
                        SubmitType::Publish => create_event::CreateEventUCKind::Private,
                        SubmitType::Save => create_event::CreateEventUCKind::Draft,
                    },
                    title: input.title.clone(),
                    description: input.description.clone(),
                    slots: input
                        .slots
                        .iter()
                        .map(|s| event::EventSlot {
                            jobs: s.jobs.clone(),
                        })
                        .collect::<Vec<_>>(),
                    start_at: input.start_at,
                    deadline_at: input.deadline_at,
                    duration: Duration::hours(2),
                },
            )
            .await?;
        println!("{:?}", event);

        Ok(HttpResponse::Created()
            .append_header(HxLocation("/"))
            .finish())
    }

    async fn page(
        access_type: AccessType,
        discord_req: Data<infra::DiscordReq>,
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
