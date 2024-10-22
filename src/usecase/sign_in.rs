use chrono::Duration;

use crate::domain::{
    auth::{SessionCreateInput, SessionRepo, SessionTokenService},
    member::{DiscordService, MemberRepo},
    Error,
};

pub struct SignInUC<MemberR, SessionR, SessionTokenS, DiscordS> {
    pub member_repo: MemberR,
    pub session_repo: SessionR,

    pub session_token_service: SessionTokenS,
    pub discord_service: DiscordS,
}

impl<MemberR, SessionR, SessionTokenS, DiscordR>
    SignInUC<MemberR, SessionR, SessionTokenS, DiscordR>
where
    MemberR: MemberRepo,
    SessionR: SessionRepo,
    SessionTokenS: SessionTokenService,
    DiscordR: DiscordService,
{
    pub fn new(
        member_repo: MemberR,
        session_repo: SessionR,
        session_token_service: SessionTokenS,
        discord_service: DiscordR,
    ) -> Self {
        Self {
            member_repo,
            session_repo,
            session_token_service,
            discord_service,
        }
    }

    /// Execute sign in with discord code
    /// Return the session token
    pub async fn execute(&self, discord_code: &str, redirect_uri: &str) -> Result<String, Error> {
        // Sign in discord
        let discord_member = self
            .discord_service
            .sign_in(discord_code, redirect_uri)
            .await?;
        // Insert or update member
        let member_insert_input = self
            .discord_service
            .into_member_insert_input(&discord_member)?;
        let member = self.member_repo.insert(member_insert_input).await?;

        // Create new session
        let session = self
            .session_repo
            .create(SessionCreateInput::new(member.id, Duration::weeks(1)))
            .await?;
        self.session_token_service
            .sign_session(&session.id)
            .await
            .map(|token| token.to_string())
    }
}
