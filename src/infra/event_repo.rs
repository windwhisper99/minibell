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
            event::EventStatus::InProcess => Self::InProgress,
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
            Self::InProgress => event::EventStatus::InProcess,
            Self::Finished => event::EventStatus::Finished,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct EventSlotModel {
    jobs: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct EventModel {
    id: String,
    title: String,
    description: Option<String>,

    status: EventStatusModel,

    host: Option<u64>,

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
            id: value.id.to_string(),
            title: value.info.title.clone(),
            description: value.info.description.clone(),

            status: value.status.clone().into(),

            slots: value
                .slots
                .iter()
                .map(|s| EventSlotModel {
                    jobs: s.jobs.iter().map(|j| j.to_string()).collect::<Vec<_>>(),
                })
                .collect(),

            start_at: value.schedule.start_at,
            deadline_at: value.schedule.deadline_at,
            duration: value.schedule.duration.num_minutes(),

            host: match value.host {
                event::EventHost::Member(id) => Some(id),
                event::EventHost::System => None,
            },

            created_at: value.created_at,
            updated_at: value.updated_at,
            published_at: value.published_at,
        }
    }
}

impl Into<event::Event> for EventModel {
    fn into(self) -> event::Event {
        event::Event {
            id: self.id.clone(),
            info: event::EventInfo {
                title: self.title,
                description: self.description,
            },
            status: self.status.into(),
            host: match self.host {
                Some(key) => event::EventHost::Member(key),
                None => event::EventHost::System,
            },
            slots: self
                .slots
                .iter()
                .map(|s| event::EventSlot {
                    jobs: s.jobs.iter().map(|j| j.to_string()).collect::<Vec<_>>(),
                })
                .collect(),
            schedule: event::EventSchedule {
                start_at: self.start_at,
                deadline_at: self.deadline_at,
                duration: chrono::Duration::minutes(self.duration),
            },
            created_at: self.created_at,
            updated_at: self.updated_at,
            published_at: self.published_at,
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

static EVENT_FIELDS: &str = r#"
    *,
    id.id(),
    (host && host.id()) as host,
    slots.{jobs:jobs.map(|$j|$j.id())}"#;

impl event::EventRepo for &EventRepo {
    async fn insert(
        &self,
        event: &event::Event,
        log: &Option<event::EventLog>,
    ) -> Result<(), Error> {
        let query = self.db.query("BEGIN TRANSACTION").query(
            "INSERT INTO event {
                id: $id,
                title: $title,
                description: $description,

                status: $status,

                host: $host && type::thing('member',$host),

                slots: $slots.map(|$s| {
                    jobs: $s.jobs.map(|$j| type::thing('job',$j))
                }),

                start_at: <datetime>$start_at,
                deadline_at: $deadline_at && <datetime>$deadline_at,
                duration: $duration,

                created_at: <datetime>$created_at,
                updated_at: <datetime>$updated_at,
                published_at: $published_at && <datetime>$published_at
            }
            ON DUPLICATE KEY UPDATE
                title = $input.title,
                description = $input.description,
                status = $input.status,

                slots = $input.slots,

                start_at = $input.start_at,
                deadline_at = $input.deadline_at,
                duration = $input.duration,

                updated_at = $input.updated_at,
                published_at = $input.published_at",
        );

        let query = if let Some(log) = log {
            query
                .query(
                    "CREATE ONLY event_log SET
                        event = type::thing('event',$id),
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

    async fn get_by_id(&self, id: &str) -> Result<event::Event, Error> {
        self.db
            .query(format!(
                "SELECT {} FROM ONLY event WHERE id = $id LIMIT 1",
                EVENT_FIELDS
            ))
            .bind(("id", RecordId::from_table_key("event", id)))
            .await?
            .take::<Option<EventModel>>(0)?
            .ok_or_else(|| Error::NotFound)
            .map(Into::into)
    }
}
