use actix_web::{
    web::{get, post, scope, Data, Json, ServiceConfig},
    HttpResponse, Responder,
};
use askama::Template;
use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::utils::HxLocation;

mod create {
    use actix_web::web::Query;
    use chrono::Duration;
    use serde::Serialize;
    use serde_with::{serde_as, TimestampMilliSeconds};

    use crate::{
        domain::{self, auth::AccessType, event, Error},
        infra,
        usecase::{draft_event, get_draft_event},
        // web::utils::{
        //     authorizated_check,
        //     templates::{self, datetime_to_string},
        // },
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

        submit: SubmitType,
        is_private: bool,
    }

    impl Into<draft_event::DraftEventUCInput> for DraftEventInput {
        fn into(self) -> draft_event::DraftEventUCInput {
            draft_event::DraftEventUCInput {
                kind: match &self.submit {
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

    #[derive(Template)]
    #[template(path = "edit_event.html")]
    struct EditEventTemplate {
        user_status: templates::UserStatusTempl,
        editing_event: Option<String>,

        title: String,
        description: String,

        start_at: String,
        deadline_at: String,
        duration: String,

        slots: String,
    }

    impl EditEventTemplate {
        fn new(
            access_type: &AccessType,
            discord_req: &infra::DiscordReq,
            event: Option<domain::event::Event>,
        ) -> Self {
            let user_status = templates::UserStatusTempl::new(access_type, discord_req);

            #[derive(Debug, Serialize)]
            struct Slot {
                jobs: Vec<String>,
            }

            let slots = match &event {
                Some(event) => event
                    .slots
                    .iter()
                    .map(|s| Slot {
                        jobs: s.jobs.clone(),
                    })
                    .collect::<Vec<_>>(),
                None => vec![Slot { jobs: vec![] }],
            };

            match event {
                Some(event) => Self {
                    user_status,
                    editing_event: Some(event.id.clone()),

                    title: event.info.title.clone(),
                    description: event.info.description.unwrap_or_default(),

                    start_at: datetime_to_string(event.schedule.start_at),
                    deadline_at: event
                        .schedule
                        .deadline_at
                        .map_or("".to_string(), datetime_to_string),
                    duration: event.schedule.duration.num_minutes().to_string(),

                    slots: serde_json::to_string(&slots).unwrap(),
                },
                None => Self {
                    user_status,
                    editing_event: None,

                    title: "".to_string(),
                    description: "".to_string(),

                    start_at: "".to_string(),
                    deadline_at: "".to_string(),
                    duration: "120".to_string(),

                    slots: serde_json::to_string(&slots).unwrap(),
                },
            }
        }
    }

    async fn submit(
        access_type: AccessType,
        input: Json<DraftEventInput>,
        event_repo: Data<infra::EventRepo>,
    ) -> Result<impl Responder, Error> {
        authorizated_check(&access_type)?;
        println!("{:#?}", &input.0);

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

        if event.status.is_draft() {
            Ok(HttpResponse::Ok()
                .append_header(HxLocation(format!("/events/create?id={}", event.id)))
                .finish())
        } else {
            Ok(HttpResponse::Ok().append_header(HxLocation("/")).finish())
        }
    }

    #[derive(Debug, Deserialize)]
    struct CreatePageQuery {
        id: Option<String>,
    }

    async fn page(
        access_type: AccessType,
        discord_req: Data<infra::DiscordReq>,
        event_repo: Data<infra::EventRepo>,
        query: Query<CreatePageQuery>,
    ) -> Result<impl Responder, Error> {
        authorizated_check(&access_type)?;

        match &query.id {
            Some(id) => {
                let event = get_draft_event::GetDraftEventUC::new(event_repo.as_ref())
                    .execute(&access_type, id)
                    .await?;

                Ok(EditEventTemplate::new(
                    &access_type,
                    discord_req.as_ref(),
                    Some(event),
                ))
            }
            None => Ok(EditEventTemplate::new(
                &access_type,
                discord_req.get_ref(),
                None,
            )),
        }
    }

    pub fn config(cfg: &mut ServiceConfig) {
        cfg.route("create", get().to(page))
            .route("create", post().to(submit));
    }
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("events").configure(create::config));
}
