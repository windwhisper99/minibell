use chrono::{DateTime, Duration, Utc};

use super::{
    auth::AccessType,
    member::{Member, MemberId},
    Error,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventStatus {
    Draft,
    Private,
    Public,
    InProgress,
    Finished,
}

impl EventStatus {
    pub fn is_draft(&self) -> bool {
        self == &EventStatus::Draft
    }

    pub fn is_published(&self) -> bool {
        self != &EventStatus::Draft
    }

    pub fn is_started(&self) -> bool {
        self == &EventStatus::InProgress || self == &EventStatus::Finished
    }

    pub fn is_in_progress(&self) -> bool {
        self == &EventStatus::InProgress
    }
}

#[derive(Debug, Clone)]
pub struct EventSlot {
    pub jobs: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum EventHost {
    Member(MemberId),
    System,
}

impl EventHost {
    /// Verify if the access type can write to the event
    fn verify_write_access(&self, access_type: &AccessType) -> Result<(), Error> {
        match access_type {
            AccessType::Unauthenticated => Err(Error::Unauthenticated),
            // System access can write to any event
            AccessType::System => Ok(()),
            AccessType::Session(session_with_member) => {
                if let EventHost::Member(host_id) = self {
                    if host_id == &session_with_member.member.id {
                        return Ok(());
                    }
                }

                Err(Error::Forbidden)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct EventInfo {
    pub title: String,
    pub description: Option<String>,
}

impl EventInfo {
    fn validate(&self) -> Result<(), Error> {
        // Title must not be empty
        if self.title.is_empty() {
            return Err(Error::BadRequest("Title must not be empty".to_string()));
        }
        // Title must not exceed 100 characters
        if self.title.len() > 100 {
            return Err(Error::BadRequest(
                "Title must not exceed 100 characters".to_string(),
            ));
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct EventSchedule {
    pub start_at: DateTime<Utc>,
    pub deadline_at: Option<DateTime<Utc>>,
    pub duration: Duration,
}

impl EventSchedule {
    fn validate(&self) -> Result<(), Error> {
        // Start time must be in the future at least 15 minutes
        if self.start_at < Utc::now() + Duration::minutes(15) {
            return Err(Error::BadRequest(
                "Start time must be in the future".to_string(),
            ));
        }
        // Deadline time must be in the future at least 15 minutes and before start time
        if let Some(deadline_at) = self.deadline_at {
            if deadline_at < Utc::now() + Duration::minutes(15) || deadline_at > self.start_at {
                return Err(Error::BadRequest(
                    "Deadline time must be in the future".to_string(),
                ));
            }
        }

        // Verify duration, must be at least 15 minutes and not exceed 24 hours
        if self.duration < Duration::minutes(15) || self.duration > Duration::hours(24) {
            return Err(Error::BadRequest(
                "Duration must be at least 15 minutes".to_string(),
            ));
        }

        Ok(())
    }

    fn is_started(&self) -> bool {
        self.start_at <= Utc::now()
    }

    fn is_finished(&self) -> bool {
        Utc::now() >= self.start_at + self.duration
    }
}

#[derive(Debug, Clone)]
pub struct Event {
    pub id: String,
    pub host: EventHost,

    pub status: EventStatus,
    pub info: EventInfo,

    pub slots: Vec<EventSlot>,

    pub schedule: EventSchedule,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub enum EventLogKind {
    Publish,
    Info {
        old: EventInfo,
        new: EventInfo,
    },
    Schedule {
        old: EventSchedule,
        new: EventSchedule,
    },
    Start,
    StartManually,
    End,
    EndManually,
}

#[derive(Debug, Clone)]
pub struct EventLog {
    pub kind: EventLogKind,
    pub at: DateTime<Utc>,
}

pub struct EventDraftInput {
    pub info: EventInfo,
    pub slots: Vec<EventSlot>,
    pub schedule: EventSchedule,
}

pub enum EventPublishInput {
    Private,
    Public,
}

/// Use for editing published event
pub enum EventEditInput {
    Info(EventInfo),
    Schedule(EventSchedule),
}

/// Use for starting the event
pub enum EventStartInput {
    Manually(Member),
    Auto,
}

/// Use for finishing the event
pub enum EventFinishInput {
    Manually(Member),
    Auto,
}

impl Event {
    /// Create new draft event
    pub fn new(access_type: &AccessType, input: EventDraftInput) -> Result<Self, Error> {
        let now = Utc::now();
        let id = sqids::Sqids::default()
            .encode(&[now.timestamp_millis() as u64])
            .map_err(|e| Error::Internal(e.to_string()))?;

        let host = match access_type {
            AccessType::Unauthenticated => return Err(Error::Unauthenticated),
            AccessType::System => EventHost::System,
            AccessType::Session(session_with_member) => {
                EventHost::Member(session_with_member.member.id)
            }
        };

        Ok(Event {
            id,
            host,

            status: EventStatus::Draft,
            info: input.info,
            slots: input.slots,
            schedule: input.schedule,

            created_at: now,
            updated_at: now,
            published_at: None,
        })
    }

    fn is_writeable(&self, access_type: &AccessType) -> bool {
        match self.host {
            EventHost::Member(host_id) => match access_type {
                AccessType::System => true,
                AccessType::Session(session_with_member) => {
                    host_id == session_with_member.member.id
                }
                _ => false,
            },
            EventHost::System => match access_type {
                AccessType::System => true,
                _ => false,
            },
        }
    }

    /// Check if the event is updateable by the access type
    pub fn is_updateable(&self, access_type: &AccessType) -> bool {
        // Can only update draft event
        self.status.is_draft() && self.is_writeable(access_type)
    }

    /// Update event with log
    /// Update the updated_at field
    fn update_with_log(&mut self, kind: EventLogKind) -> EventLog {
        let at = Utc::now();
        self.updated_at = at;

        EventLog { kind, at }
    }

    /// Update draft event
    pub fn update(
        &mut self,
        access_type: &AccessType,
        input: EventDraftInput,
    ) -> Result<(), Error> {
        self.host.verify_write_access(access_type)?;
        // Can only update draft event
        if !self.status.is_draft() {
            return Err(Error::BadRequest("Can only update draft event".to_string()));
        }

        self.info = input.info;
        self.slots = input.slots;
        self.schedule = input.schedule;
        self.updated_at = Utc::now();

        Ok(())
    }

    /// Publish a draft event
    /// TODO: also return sign up data for the host
    pub fn publish(
        &mut self,
        access_type: &AccessType,
        kind: EventPublishInput,
    ) -> Result<EventLog, Error> {
        self.host.verify_write_access(access_type)?;
        // Event must be in draft status
        if self.status != EventStatus::Draft {
            return Err(Error::BadRequest(
                "Can only publish draft event".to_string(),
            ));
        }

        // Verify event data
        self.validate()?;
        self.status = match kind {
            EventPublishInput::Private => EventStatus::Private,
            EventPublishInput::Public => EventStatus::Public,
        };

        let log = self.update_with_log(EventLogKind::Publish);
        self.published_at = Some(log.at);
        Ok(log)
    }

    /// Edit the event
    pub fn edit(
        &mut self,
        access_type: &AccessType,
        input: EventEditInput,
    ) -> Result<EventLog, Error> {
        self.host.verify_write_access(access_type)?;
        // Can't edit draft event or started event
        if self.status.is_draft() || self.status.is_started() {
            return Err(Error::BadRequest(
                "Can't edit draft or started event".to_string(),
            ));
        }

        match input {
            EventEditInput::Info(info) => {
                // Verify info
                info.validate()?;
                let old = self.info.clone();
                self.info = info.clone();
                Ok(self.update_with_log(EventLogKind::Info { old, new: info }))
            }
            EventEditInput::Schedule(schedule) => {
                // Verify schedule
                schedule.validate()?;
                let old = self.schedule.clone();
                self.schedule = schedule.clone();
                Ok(self.update_with_log(EventLogKind::Schedule { old, new: schedule }))
            }
        }
    }

    /// Start the event
    pub fn start(
        &mut self,
        access_type: &AccessType,
        input: EventStartInput,
    ) -> Result<EventLog, Error> {
        self.host.verify_write_access(access_type)?;
        // Event much be published and not started
        if !self.status.is_published() || self.status.is_started() {
            return Err(Error::BadRequest(
                "Can only start published event".to_string(),
            ));
        }

        match input {
            EventStartInput::Manually(member) => {
                // Only member can start the event
                if let EventHost::Member(host_id) = &self.host {
                    if host_id != &member.id {
                        return Err(Error::Forbidden);
                    }
                } else {
                    return Err(Error::Forbidden);
                }

                self.status = EventStatus::InProgress;
                Ok(self.update_with_log(EventLogKind::StartManually))
            }
            EventStartInput::Auto => {
                // Check if the schedule is started
                if !self.schedule.is_started() {
                    return Err(Error::BadAutomation);
                }

                self.status = EventStatus::InProgress;
                Ok(self.update_with_log(EventLogKind::Start))
            }
        }
    }

    /// Finished the event
    pub fn finish(
        &mut self,
        access_type: &AccessType,
        input: EventFinishInput,
    ) -> Result<EventLog, Error> {
        self.host.verify_write_access(access_type)?;
        // Can only finish in progress event
        if !self.status.is_in_progress() {
            return Err(Error::BadRequest(
                "Can only finish in progress event".to_string(),
            ));
        }

        match input {
            EventFinishInput::Manually(member) => {
                // Only member can end the event
                if let EventHost::Member(host_id) = &self.host {
                    if host_id != &member.id {
                        return Err(Error::Forbidden);
                    }
                } else {
                    return Err(Error::Forbidden);
                }

                self.status = EventStatus::Finished;
                Ok(self.update_with_log(EventLogKind::EndManually))
            }
            EventFinishInput::Auto => {
                // Check if the schedule is finish
                if !self.schedule.is_finished() {
                    return Err(Error::BadAutomation);
                }

                self.status = EventStatus::Finished;
                Ok(self.update_with_log(EventLogKind::End))
            }
        }
    }

    // Verify event data
    pub fn validate(&self) -> Result<(), Error> {
        self.info.validate()?;

        // Slots must have at least 2 slots
        if self.slots.len() < 2 {
            return Err(Error::BadRequest(
                "Slots must have at least 2 slots".to_string(),
            ));
        }

        self.schedule.validate()?;
        Ok(())
    }
}

pub trait EventRepo {
    /// Insert or update event
    async fn insert(&self, event: &Event, log: &Option<EventLog>) -> Result<(), Error>;

    /// Get event by id
    async fn get_by_id(&self, id: &str) -> Result<Event, Error>;
}
