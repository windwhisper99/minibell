use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

use crate::domain::{member, Error};

/// Discord request client service
#[derive(Clone)]
pub struct DiscordReq {
    reqwest: Arc<reqwest::Client>,

    client_id: String,
    client_secret: String,
    guild_id: u64,
    token: String,

    redirect_uri: String,
}

#[serde_as]
#[derive(Debug, Deserialize)]
struct UserPayload {
    #[serde_as(as = "DisplayFromStr")]
    id: u64,
    global_name: String,
    avatar: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct MemberPayload {
    user: UserPayload,
    nick: Option<String>,
    avatar: Option<String>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    roles: Vec<u64>,
    joined_at: DateTime<Utc>,
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::ServiceError(err.to_string())
    }
}

impl DiscordReq {
    pub fn new(reqwest: Arc<reqwest::Client>) -> Self {
        let client_id = std::env::var("DISCORD_CLIENT_ID").expect("DISCORD_CLIENT_ID must be set");
        let client_secret =
            std::env::var("DISCORD_CLIENT_SECRET").expect("DISCORD_CLIENT_SECRET must be set");
        let guild_id = std::env::var("DISCORD_GUILD_ID")
            .expect("DISCORD_GUILD_ID must be set")
            .parse::<u64>()
            .expect("DISCORD_GUILD_ID must be a number");
        let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set");
        let redirect_uri =
            std::env::var("DISCORD_REDIRECT_URI").expect("DISCORD_REDIRECT_URI must be set");

        Self {
            reqwest,

            client_id,
            client_secret,
            guild_id,
            token,

            redirect_uri,
        }
    }

    /// Fetch access token from Oauth2 code
    async fn fetch_user_tokens(&self, code: &str) -> Result<String, Error> {
        #[derive(Debug, Deserialize)]
        struct AuthorizationResult {
            access_token: String,
        }

        self.reqwest
            .post("https://discord.com/api/v10/oauth2/token")
            .form(&[
                ("client_id", self.client_id.as_str()),
                ("client_secret", self.client_secret.as_str()),
                ("grant_type", "authorization_code"),
                ("code", code),
                ("redirect_uri", self.redirect_uri.as_str()),
            ])
            .send()
            .await?
            .json::<AuthorizationResult>()
            .await
            .map(|payload| payload.access_token)
            .map_err(Into::into)
    }

    /// Fetch user id from access token
    async fn fetch_user_id(&self, access_token: &str) -> Result<u64, Error> {
        #[serde_as]
        #[derive(Deserialize)]
        struct Payload {
            #[serde_as(as = "DisplayFromStr")]
            id: u64,
        }

        self.reqwest
            .get("https://discord.com/api/v10/users/@me")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?
            .json::<Payload>()
            .await
            .map(|payload| payload.id)
            .map_err(Into::into)
    }

    /// Fetch member info by user id
    /// Update and store member information
    async fn fetch_member_info(&self, user_id: u64) -> Result<MemberPayload, Error> {
        self.reqwest
            .get(format!(
                "https://discord.com/api/v10/guilds/{}/members/{}",
                self.guild_id, user_id
            ))
            .header("Authorization", format!("Bot {}", self.token))
            .send()
            .await?
            .json::<MemberPayload>()
            .await
            .map_err(Into::into)
    }

    async fn auth(&self, code: &str) -> Result<MemberPayload, Error> {
        let access_token = self.fetch_user_tokens(code).await?;
        let user_id = self.fetch_user_id(&access_token).await?;
        self.fetch_member_info(user_id).await
    }

    fn avatar_url(&self, payload: &MemberPayload) -> String {
        if let Some(avatar) = &payload.avatar {
            format!(
                "https://cdn.discordapp.com/guilds/{}/users/{}/avatars/{}.png",
                self.guild_id, payload.user.id, avatar
            )
        } else if let Some(avatar) = &payload.user.avatar {
            format!(
                "https://cdn.discordapp.com/avatars/{}/{}.png",
                payload.user.id, avatar
            )
        } else {
            let index = (payload.user.id >> 22) % 6;
            format!("https://cdn.discordapp.com/embed/avatars/{}.png", index)
        }
    }
}

impl member::DiscordService for &DiscordReq {
    type DiscordMember = MemberPayload;

    async fn sign_in(&self, code: &str) -> Result<Self::DiscordMember, Error> {
        self.auth(code).await
    }

    async fn get_member(&self, member_id: member::MemberId) -> Result<Self::DiscordMember, Error> {
        self.fetch_member_info(member_id).await
    }

    fn into_member_insert_input(
        &self,
        discord_member: &Self::DiscordMember,
    ) -> Result<member::MemberInsertInput, Error> {
        Ok(member::MemberInsertInput {
            id: discord_member.user.id,
            display_name: discord_member
                .nick
                .clone()
                .unwrap_or(discord_member.user.global_name.clone()),
            avatar: self.avatar_url(discord_member),
            joined_at: discord_member.joined_at,
            roles: discord_member.roles.clone(),
        })
    }
}
