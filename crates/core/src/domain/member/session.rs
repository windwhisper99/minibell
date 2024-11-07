use chrono::{DateTime, Utc};

/// Sign in session
#[derive(Debug, Clone)]
pub struct MemberSession {
    pub id: String,
    pub member_id: u64,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}
