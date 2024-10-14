use chrono::{DateTime, Utc};

use super::{
    member::{Member, MemberId},
    Error,
};

/// Use for access control in the application
#[derive(Debug)]
pub enum AccessType {
    /// Access type for unauthenticated user
    Unauthenticated,
    /// Access type for CLI
    CLI,
    /// Access type for bot
    Bot,
    /// Access type for authenticated user
    Session(SessionWithMember),
}

impl AccessType {
    pub fn get_me(&self) -> Result<&Member, Error> {
        match self {
            Self::Session(session) => Ok(&session.member),
            _ => Err(Error::Unauthenticated),
        }
    }

    pub fn get_session(&self) -> Result<&Session, Error> {
        match self {
            Self::Session(session) => Ok(&session.session),
            _ => Err(Error::Unauthenticated),
        }
    }
}

/// Sign in session
#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub member_id: u64,
    pub issued_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

/// Session with member
#[derive(Debug, Clone)]
pub struct SessionWithMember {
    pub session: Session,
    pub member: Member,
}

#[derive(Debug, Clone)]
pub struct SessionCreateInput {
    pub member_id: MemberId,
    pub issued_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl SessionCreateInput {
    pub fn new(member_id: MemberId, duration: chrono::Duration) -> Self {
        let now = Utc::now();

        Self {
            member_id,
            issued_at: now,
            updated_at: now,
            expires_at: now + duration,
        }
    }
}

/// Session repository
pub trait SessionRepo {
    /// Create new session
    async fn create<I>(&self, input: I) -> Result<Session, Error>
    where
        I: Into<SessionCreateInput>;
    /// Get session by id
    async fn get(&self, session_id: &str) -> Result<SessionWithMember, Error>;
}

/// Session token
pub trait SessionToken: ToString {
    fn session_id(&self) -> &str;
}

/// Session token service
pub trait SessionTokenService {
    type SessionToken: SessionToken;

    /// Create new session token
    async fn sign_session(&self, session_id: &str) -> Result<Self::SessionToken, Error>;
    /// Verify session token and return session id
    async fn verify_token(&self, token: &str) -> Result<Self::SessionToken, Error>;
}
