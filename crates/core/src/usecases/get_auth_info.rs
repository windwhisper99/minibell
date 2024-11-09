use async_trait::async_trait;

use crate::{
    member::{DiscordClient, Member, MemberId, MemberRepository},
    Error,
};

use super::UseCase;

pub struct GetAuthInfo<'a> {
    pub discord_client: &'a dyn DiscordClient,
    pub member_repo: &'a dyn MemberRepository,
}

#[derive(Debug, Clone)]
pub struct GetAuthInfoInput {
    pub redirect_uri: String,
}

#[derive(Debug, Clone)]
pub struct GetAuthInfoResponse {
    pub auth_url: String,
    pub member: Option<Member>,
}

impl<'a> GetAuthInfo<'a> {
    async fn run(
        &self,
        member_id: Option<MemberId>,
        input: GetAuthInfoInput,
    ) -> Result<GetAuthInfoResponse, Error> {
        let auth_url = self.discord_client.get_oauth2_url(&input.redirect_uri);
        let member = match member_id {
            Some(member_id) => self.member_repo.get_member(member_id).await.ok(),
            None => None,
        };

        Ok(GetAuthInfoResponse { auth_url, member })
    }
}

#[async_trait]
impl<'a> UseCase for GetAuthInfo<'a> {
    type Input = GetAuthInfoInput;
    type Response = GetAuthInfoResponse;

    async fn guest_execute(&self, input: Self::Input) -> Result<Self::Response, Error> {
        self.run(None, input).await
    }

    async fn member_execute(
        &self,
        member_id: MemberId,
        input: Self::Input,
    ) -> Result<Self::Response, Error> {
        self.run(Some(member_id), input).await
    }
}
