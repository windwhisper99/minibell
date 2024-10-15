use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

use crate::domain::{event, Error};

use super::Database;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum EventStatusModel {
    Draft,
    Public,
    Private,
    InProgress,
    Finished,
}

impl From<event::EventStatus> for EventStatusModel {
    fn from(status: event::EventStatus) -> Self {
        match status {
            event::EventStatus::Draft => Self::Draft,
            event::EventStatus::Public => Self::Public,
            event::EventStatus::Private => Self::Private,
            event::EventStatus::InProgress => Self::InProgress,
            event::EventStatus::Finished => Self::Finished,
        }
    }
}

impl Into<event::EventStatus> for EventStatusModel {
    fn into(self) -> event::EventStatus {
        match self {
            Self::Draft => event::EventStatus::Draft,
            Self::Public => event::EventStatus::Public,
            Self::Private => event::EventStatus::Private,
            Self::InProgress => event::EventStatus::InProgress,
            Self::Finished => event::EventStatus::Finished,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct EventSlotModel {
    jobs: Vec<RecordId>,
}

#[derive(Debug, Deserialize, Serialize)]
struct EventModel {
    id: RecordId,
    title: String,
    description: Option<String>,

    status: EventStatusModel,

    host: RecordId,

    slots: Vec<EventSlotModel>,

    start_at: DateTime<Utc>,
    deadline_at: Option<DateTime<Utc>>,
    duration: i64,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    published_at: Option<DateTime<Utc>>,
}

impl From<&event::Event> for EventModel {
    fn from(value: &event::Event) -> Self {
        Self {
            id: RecordId::from_table_key("event", &value.id),
            title: value.info.title.clone(),
            description: value.info.description.clone(),

            status: value.status.clone().into(),

            slots: value
                .slots
                .iter()
                .map(|s| EventSlotModel {
                    jobs: s
                        .jobs
                        .iter()
                        .map(|j| RecordId::from_table_key("job", j))
                        .collect::<Vec<_>>(),
                })
                .collect(),

            start_at: value.schedule.start_at,
            deadline_at: value.schedule.deadline_at,
            duration: value.schedule.duration.num_minutes(),

            host: match &value.host {
                event::EventHost::Member(member) => {
                    RecordId::from_table_key("member", *member as i64)
                }
                event::EventHost::System => RecordId::from_table_key("member", "system"),
            },

            created_at: value.created_at,
            updated_at: value.updated_at,
            published_at: value.published_at,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct EventInfoModel {
    title: String,
    description: Option<String>,
}

impl From<&event::EventInfo> for EventInfoModel {
    fn from(value: &event::EventInfo) -> Self {
        Self {
            title: value.title.clone(),
            description: value.description.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct EventScheduleModel {
    start_at: DateTime<Utc>,
    deadline_at: Option<DateTime<Utc>>,
    duration: i64,
}

impl From<&event::EventSchedule> for EventScheduleModel {
    fn from(value: &event::EventSchedule) -> Self {
        Self {
            start_at: value.start_at,
            deadline_at: value.deadline_at,
            duration: value.duration.num_minutes(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", content = "content", rename_all = "snake_case")]
enum EventLogKindModel {
    Publish,
    Info {
        old: EventInfoModel,
        new: EventInfoModel,
    },
    Schedule {
        old: EventScheduleModel,
        new: EventScheduleModel,
    },
    Start,
    StartManually,
    End,
    EndManually,
}

impl From<&event::EventLogKind> for EventLogKindModel {
    fn from(value: &event::EventLogKind) -> Self {
        match value {
            event::EventLogKind::Publish => EventLogKindModel::Publish,
            event::EventLogKind::Info { old, new } => EventLogKindModel::Info {
                old: old.into(),
                new: new.into(),
            },
            event::EventLogKind::Schedule { old, new } => EventLogKindModel::Schedule {
                old: old.into(),
                new: new.into(),
            },
            event::EventLogKind::Start => EventLogKindModel::Start,
            event::EventLogKind::StartManually => EventLogKindModel::StartManually,
            event::EventLogKind::End => EventLogKindModel::End,
            event::EventLogKind::EndManually => EventLogKindModel::EndManually,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct EventLogModel {
    kind: EventLogKindModel,
    at: DateTime<Utc>,
}

impl From<&event::EventLog> for EventLogModel {
    fn from(value: &event::EventLog) -> Self {
        Self {
            kind: EventLogKindModel::from(&value.kind),
            at: value.at,
        }
    }
}

#[derive(Clone)]
pub struct EventRepo {
    db: Arc<Database>,
}

impl EventRepo {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }
}

impl event::EventRepo for &EventRepo {
    async fn create(
        &self,
        event: &event::Event,
        log: &Option<event::EventLog>,
    ) -> Result<(), Error> {
        let query = self.db.query("BEGIN TRANSACTION").query(
            "CREATE ONLY event SET
                    id = $id,

                    title = $title,
                    description = $description,
                    status = $status,
                    
                    host = $host,

                    slots = $slots,

                    start_at = <datetime>$start_at,
                    deadline_at = $deadline_at && <datetime>$deadline_at,
                    duration = $duration,
                    
                    created_at = <datetime>$created_at,
                    updated_at = <datetime>$updated_at,
                    published_at = $published_at && <datetime>$published_at",
        );

        let query = if let Some(log) = log {
            query
                .query(
                    "CREATE ONLY event_log SET
                    event = $id,
                    type = $log.kind.type,
                    content = $log.kind.content,
                    at = <datetime>$log.at",
                )
                .bind(("log", EventLogModel::from(log)))
        } else {
            query
        };

        query
            .query("COMMIT")
            .bind(EventModel::from(event))
            .await?
            .check()?;

        Ok(())
    }

    async fn update(
        &self,
        event: &event::Event,
        log: &Option<event::EventLog>,
    ) -> Result<(), Error> {
        unimplemented!()
    }

    async fn get_by_id(&self, id: &str) -> Result<event::Event, Error> {
        unimplemented!()
    }
}
