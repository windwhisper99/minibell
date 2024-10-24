use actix_web::{
    web::{get, post, scope, Data, Json, ServiceConfig},
    HttpResponse, Responder,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;

mod draft {
    use actix_web::web::Path;
    use chrono::Duration;
    use serde::Serialize;
    use serde_with::{serde_as, TimestampMilliSeconds};

    use crate::{
        api::dto::EventDto,
        domain::{auth::AccessType, event, Error},
        infra,
        usecase::{draft_event, get_draft_event},
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
    #[serde(rename_all = "camelCase")]
    struct DraftEventInput {
        editing_event: Option<String>,

        title: String,
        description: Option<String>,

        // Slots
        slots: Vec<DraftEventSlotInput>,

        // Schedule
        #[serde_as(as = "TimestampMilliSeconds<i64>")]
        start_at: DateTime<Utc>,
        #[serde_as(as = "Option<TimestampMilliSeconds<i64>>")]
        deadline_at: Option<DateTime<Utc>>,
        duration: i64,

        submit_type: SubmitType,
        is_private: bool,
    }

    impl Into<draft_event::DraftEventUCInput> for DraftEventInput {
        fn into(self) -> draft_event::DraftEventUCInput {
            draft_event::DraftEventUCInput {
                kind: match &self.submit_type {
                    SubmitType::Publish => match self.is_private {
                        true => draft_event::DraftEventUCKind::Private,
                        false => draft_event::DraftEventUCKind::Public,
                    },
                    SubmitType::Save => draft_event::DraftEventUCKind::Draft,
                },
                title: self.title.clone(),
                description: self.description.clone(),
                slots: self
                    .slots
                    .iter()
                    .map(|s| event::EventSlot {
                        jobs: s.jobs.clone(),
                    })
                    .collect::<Vec<_>>(),
                start_at: self.start_at,
                deadline_at: self.deadline_at,
                duration: Duration::minutes(self.duration),
            }
        }
    }

    #[derive(Debug, Deserialize)]
    struct DraftEventSlotInput {
        jobs: Vec<String>,
    }

    async fn submit(
        access_type: AccessType,
        input: Json<DraftEventInput>,
        event_repo: Data<infra::EventRepo>,
    ) -> Result<impl Responder, Error> {
        println!("{:?}", input);

        let event = match &input.editing_event {
            Some(id) => {
                draft_event::DraftEventUC::new(event_repo.as_ref())
                    .execute(
                        &access_type,
                        Some(id.to_string()),
                        input.into_inner().into(),
                    )
                    .await?
            }
            None => {
                draft_event::DraftEventUC::new(event_repo.as_ref())
                    .execute(&access_type, None, input.into_inner().into())
                    .await?
            }
        };

        #[derive(Serialize)]
        struct Response {
            id: String,
            published: bool,
        }

        Ok(HttpResponse::Created().json(Response {
            id: event.id,
            published: event.status.is_published(),
        }))
    }

    #[derive(Debug, Deserialize)]
    struct GetDraftParams {
        event_id: String,
    }

    async fn get_draft(
        access_type: AccessType,
        event_repo: Data<infra::EventRepo>,
        params: Path<GetDraftParams>,
    ) -> Result<impl Responder, Error> {
        let event = get_draft_event::GetDraftEventUC::new(event_repo.as_ref())
            .execute(&access_type, &params.event_id)
            .await?;

        Ok(HttpResponse::Ok().json(EventDto::from(event)))
    }

    pub fn config(cfg: &mut ServiceConfig) {
        cfg.route("draft/{event_id}", get().to(get_draft))
            .route("draft", post().to(submit));
    }
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("events").configure(draft::config));
}
