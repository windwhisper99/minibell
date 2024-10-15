use chrono::{DateTime, Duration, Utc};

use crate::domain::{
    auth::AccessType,
    event::{
        Event, EventDraftInput, EventInfo, EventPublishInput, EventRepo, EventSchedule, EventSlot,
    },
    Error,
};

pub struct DraftEventUC<EventRepo> {
    pub event_repo: EventRepo,
}

pub enum DraftEventUCKind {
    Draft,
    Public,
    Private,
}

pub struct DraftEventUCInput {
    pub kind: DraftEventUCKind,

    pub title: String,
    pub description: Option<String>,

    pub slots: Vec<EventSlot>,

    pub start_at: DateTime<Utc>,
    pub deadline_at: Option<DateTime<Utc>>,
    pub duration: Duration,
}

impl<EventR> DraftEventUC<EventR>
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
        id: Option<String>,
        input: DraftEventUCInput,
    ) -> Result<Event, Error> {
        let kind = input.kind;
        let input = EventDraftInput {
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
        };

        let mut event = match id {
            Some(id) => {
                let mut event = self.event_repo.get_by_id(&id).await?;
                event.update(access_type, input)?;
                event
            }
            None => Event::new(access_type, input)?,
        };

        let log = match kind {
            DraftEventUCKind::Draft => None,
            DraftEventUCKind::Public | DraftEventUCKind::Private => Some(event.publish(
                access_type,
                match kind {
                    DraftEventUCKind::Public => EventPublishInput::Public,
                    DraftEventUCKind::Private => EventPublishInput::Private,
                    _ => unreachable!(),
                },
            )?),
        };

        self.event_repo.insert(&event, &log).await?;
        Ok(event)
    }
}
