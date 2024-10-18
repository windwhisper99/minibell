use chrono::{DateTime, Utc};

use super::Error;

pub type MemberId = u64;
pub type RoleId = u64;

#[derive(Debug, Clone)]
pub struct Member {
    pub id: MemberId,
    pub display_name: String,
    pub avatar: String,
    pub joined_at: DateTime<Utc>,
    pub roles: Vec<RoleId>,

    /// Last time the member profile was updated
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct MemberInsertInput {
    pub id: u64,
    pub display_name: String,
    pub avatar: String,
    pub joined_at: DateTime<Utc>,
    pub roles: Vec<u64>,
}

/// Member repository
pub trait MemberRepo {
    /// Insert or update member
    async fn insert<M>(&self, input: M) -> Result<Member, Error>
    where
        M: Into<MemberInsertInput>;
}

/// Handling discord related operations
pub trait DiscordService {
    type DiscordMember;

    /// Sign in with discord Oauth2 code
    async fn sign_in(&self, code: &str) -> Result<Self::DiscordMember, Error>;
    /// Get member by given id
    async fn get_member(&self, member_id: MemberId) -> Result<Self::DiscordMember, Error>;

    /// Into member insert input
    fn into_member_insert_input(
        &self,
        discord_member: &Self::DiscordMember,
    ) -> Result<MemberInsertInput, Error>;

    /// Get discord Oauth2 URL
    fn get_oauth2_url(&self, redirect_url: &str) -> String;
}
