use chrono::{DateTime, Utc};

// mod discord;
mod repo;
mod session;

// pub use discord::DiscordClient;
pub use repo::*;
pub use session::*;

pub type MemberId = u64;

#[derive(Debug, Clone)]
pub struct Member {
    pub id: MemberId,
    pub display_name: String,
    pub avatar: String,

    /// Last time the member profile was updated
    pub updated_at: DateTime<Utc>,
    pub joined_at: DateTime<Utc>,
}

impl Member {
    pub fn new(
        id: MemberId,
        display_name: String,
        avatar: String,
        joined_at: DateTime<Utc>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id,
            display_name,
            avatar,
            updated_at: now,
            joined_at,
        }
    }
}
