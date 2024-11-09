use async_trait::async_trait;

use crate::{member::DiscordClient, Error};

use super::UseCase;

pub struct GetAuthInfo<'a> {
    pub discord_client: &'a dyn DiscordClient,
}

#[derive(Debug, Clone)]
pub struct GetAuthInfoInput {
    pub redirect_uri: String,
}

#[derive(Debug, Clone)]
pub struct GetAuthInfoResponse {
    pub auth_url: String,
}

impl<'a> GetAuthInfo<'a> {
    async fn run(&self, input: GetAuthInfoInput) -> Result<GetAuthInfoResponse, Error> {
        let auth_url = self.discord_client.get_oauth2_url(&input.redirect_uri);
        Ok(GetAuthInfoResponse { auth_url })
    }
}

#[async_trait]
impl<'a> UseCase for GetAuthInfo<'a> {
    type Input = GetAuthInfoInput;
    type Response = GetAuthInfoResponse;

    async fn guest_execute(&self, input: Self::Input) -> Result<Self::Response, Error> {
        self.run(input).await
    }
}
