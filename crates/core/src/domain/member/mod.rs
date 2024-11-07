use chrono::{DateTime, Utc};

mod session;

pub use session::*;

pub type MemberId = u64;

#[derive(Debug, Clone)]
pub struct Member {
    pub id: MemberId,
    pub display_name: String,
    pub avatar: String,

    /// Last time the member profile was updated
    pub updated_at: DateTime<Utc>,
    pub actived_at: DateTime<Utc>,
}

impl Member {
    pub fn new(id: MemberId, display_name: String, avatar: String) -> Self {
        Self {
            id,
            display_name,
            avatar,
            updated_at: Utc::now(),
            actived_at: Utc::now(),
        }
    }

    pub fn update(&mut self, display_name: String, avatar: String) {
        self.display_name = display_name;
        self.avatar = avatar;
        self.updated_at = Utc::now();
    }
}
