use crate::domain::{
    auth::AccessType,
    event::{Event, EventRepo},
    Error,
};

/// Get event for editing
/// Verify access type
pub struct GetDraftEventUC<EventRepo> {
    pub event_repo: EventRepo,
}

impl<EventR> GetDraftEventUC<EventR>
where
    EventR: EventRepo,
{
    pub fn new(event_repo: EventR) -> Self {
        Self { event_repo }
    }

    /// Create new event
    /// Support for auto public
    pub async fn execute(&self, access_type: &AccessType, event_id: &str) -> Result<Event, Error> {
        let event = self.event_repo.get_by_id(event_id).await?;

        // Check if user has access to edit
        if !event.is_updateable(access_type) {
            return Err(Error::Forbidden);
        }

        Ok(event)
    }
}
