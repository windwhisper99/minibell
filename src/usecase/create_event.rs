use chrono::{DateTime, Duration, Utc};

use crate::domain::{
    auth::AccessType,
    event::{
        Event, EventDraftInput, EventInfo, EventPublishInput, EventRepo, EventSchedule, EventSlot,
    },
    Error,
};

pub struct CreateEventUC<EventRepo> {
    pub event_repo: EventRepo,
}

pub enum CreateEventUCKind {
    Draft,
    Public,
    Private,
}

pub struct CreateEventUCInput {
    pub kind: CreateEventUCKind,

    pub title: String,
    pub description: Option<String>,

    pub slots: Vec<EventSlot>,

    pub start_at: DateTime<Utc>,
    pub deadline_at: Option<DateTime<Utc>>,
    pub duration: Duration,
}

impl<EventR> CreateEventUC<EventR>
where
    EventR: EventRepo,
{
    pub fn new(event_repo: EventR) -> Self {
        Self { event_repo }
    }

    /// Create new event
    /// Support for auto public
    pub async fn execute(
        &self,
        access_type: &AccessType,
        input: CreateEventUCInput,
    ) -> Result<Event, Error> {
        let mut event = Event::new(
            access_type,
            EventDraftInput {
                info: EventInfo {
                    title: input.title,
                    description: input.description,
                },
                slots: input.slots,
                schedule: EventSchedule {
                    start_at: input.start_at,
                    deadline_at: input.deadline_at,
                    duration: input.duration,
                },
            },
        )?;

        let log = match input.kind {
            CreateEventUCKind::Draft => None,
            CreateEventUCKind::Public | CreateEventUCKind::Private => {
                println!("Publishing event");
                let log = event.publish(
                    access_type,
                    match input.kind {
                        CreateEventUCKind::Public => EventPublishInput::Public,
                        CreateEventUCKind::Private => EventPublishInput::Private,
                        _ => unreachable!(),
                    },
                )?;

                println!("{:?}", log);
                Some(log)
            }
        };

        self.event_repo.create(&event, &log).await?;
        Ok(event)
    }
}
