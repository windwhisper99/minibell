use serde::Serialize;

use crate::domain;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberDto {
    pub id: u64,
    pub name: String,
    pub avatar: String,
}

impl From<domain::member::Member> for MemberDto {
    fn from(member: domain::member::Member) -> Self {
        Self {
            id: member.id,
            name: member.display_name,
            avatar: member.avatar,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DraftEventSlotDto {
    pub jobs: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EventStatusDto {
    Draft,
    Public,
    Private,
    InProcess,
    Finished,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventDto {
    pub id: String,

    pub status: EventStatusDto,

    pub title: String,
    pub description: Option<String>,
    pub slots: Vec<DraftEventSlotDto>,

    pub start_at: i64,
    pub deadline_at: Option<i64>,

    pub duration: i64,
}

impl From<domain::event::Event> for EventDto {
    fn from(value: domain::event::Event) -> Self {
        Self {
            id: value.id,

            status: match value.status {
                domain::event::EventStatus::Draft => EventStatusDto::Draft,
                domain::event::EventStatus::Public => EventStatusDto::Public,
                domain::event::EventStatus::Private => EventStatusDto::Private,
                domain::event::EventStatus::InProcess => EventStatusDto::InProcess,
                domain::event::EventStatus::Finished => EventStatusDto::Finished,
            },

            title: value.info.title,
            description: value.info.description,

            slots: value
                .slots
                .into_iter()
                .map(|s| DraftEventSlotDto { jobs: s.jobs })
                .collect(),

            start_at: value.schedule.start_at.timestamp_millis(),
            deadline_at: value.schedule.deadline_at.map(|d| d.timestamp_millis()),
            duration: value.schedule.duration.num_minutes(),
        }
    }
}
