use async_trait::async_trait;
use chrono::Duration;

use crate::{
    member::{DiscordClient, MemberRepository, MemberSession, MemberSessionSigner},
    Error,
};

use super::UseCase;

pub struct SignIn<'a> {
    pub discord_client: &'a dyn DiscordClient,
    pub member_repo: &'a dyn MemberRepository,
    pub member_session_signer: &'a dyn MemberSessionSigner,
}

pub struct SignInInput {
    pub discord_code: String,
    pub redirect_uri: String,
}

impl<'a> SignIn<'a> {
    async fn run(&self, input: SignInInput) -> Result<String, Error> {
        let member = self
            .discord_client
            .sign_in(&input.discord_code, &input.redirect_uri)
            .await?;
        let session = MemberSession::new(member.id, Duration::days(30));

        self.member_repo
            .insert_member_and_session(&member, &session)
            .await?;

        let token = self.member_session_signer.sign(&session.id)?;
        Ok(token)
    }
}

#[async_trait]
impl<'a> UseCase for SignIn<'a> {
    type Input = SignInInput;
    type Response = String;

    async fn guest_execute(&self, input: Self::Input) -> Result<Self::Response, Error> {
        self.run(input).await
    }
}
