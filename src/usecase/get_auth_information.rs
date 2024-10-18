use crate::domain::{
    auth::AccessType,
    member::{DiscordService, Member},
    Error,
};

/// Get event for editing
/// Verify access type
pub struct GetAuthInformation<DiscordS> {
    pub discord_service: DiscordS,
}

#[derive(Debug)]
pub struct AuthInformation {
    pub discord_oauth_url: String,
    pub member: Option<Member>,
}

pub struct GetAuthInformationInput {
    pub redirect_uri: String,
}

impl<DiscordS> GetAuthInformation<DiscordS>
where
    DiscordS: DiscordService,
{
    pub fn new(discord_service: DiscordS) -> Self {
        Self { discord_service }
    }

    /// Create new event
    /// Support for auto public
    pub async fn execute(
        &self,
        input: GetAuthInformationInput,
        access_type: &AccessType,
    ) -> Result<AuthInformation, Error> {
        let discord_oauth_url = self.discord_service.get_oauth2_url(&input.redirect_uri);

        Ok(AuthInformation {
            discord_oauth_url,
            member: match access_type {
                AccessType::Unauthenticated => None,
                AccessType::System => None,
                AccessType::Session(session_with_member) => {
                    Some(session_with_member.member.clone())
                }
            },
        })
    }
}
