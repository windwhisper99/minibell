use async_trait::async_trait;
use shaku::Interface;

use crate::Error;

use super::{Member, MemberId, MemberSession};

#[async_trait]
pub trait MemberRepository: Interface {
    async fn insert_member_and_session(
        &self,
        member: &Member,
        session: &MemberSession,
    ) -> Result<(), Error>;
}

#[async_trait]
pub trait MemberSessionSigner: Interface {
    /// Sign token
    fn sign(&self, session_id: &str) -> Result<String, Error>;
    /// Verify token
    fn verify(&self, token: &str) -> Result<String, Error>;
}

/// Handling discord related operations
#[async_trait]
pub trait DiscordClient: Interface {
    /// Sign in with discord Oauth2 code
    async fn sign_in(&self, code: &str, redirect_uri: &str) -> Result<Member, Error>;
    /// Get member by given id
    async fn get_member(&self, member_id: MemberId) -> Result<Member, Error>;

    /// Get discord Oauth2 URL
    fn get_oauth2_url(&self, redirect_uri: &str) -> String;
}
