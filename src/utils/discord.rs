use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

use crate::repos::{member, RepoError};

use super::db::Database;

#[derive(Debug)]
pub enum DiscordError {
    SendError(reqwest::Error),
    RepoError(RepoError),
}

impl From<RepoError> for DiscordError {
    fn from(value: RepoError) -> Self {
        DiscordError::RepoError(value)
    }
}

impl From<reqwest::Error> for DiscordError {
    fn from(value: reqwest::Error) -> Self {
        DiscordError::SendError(value)
    }
}

pub struct DiscordClient {
    client_id: String,
    client_secret: String,
    guild_id: u64,
    token: String,

    redirect_uri: String,
}

impl DiscordClient {
    pub fn new() -> Self {
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
            client_id,
            client_secret,
            guild_id,
            token,

            redirect_uri,
        }
    }

    /// Fetch access token from Oauth2 code
    async fn fetch_user_tokens(
        &self,
        client: &reqwest::Client,
        code: &str,
    ) -> Result<String, DiscordError> {
        #[derive(Debug, Deserialize)]
        struct AuthorizationResult {
            access_token: String,
        }

        client
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
    async fn fetch_user_id(
        &self,
        client: &reqwest::Client,
        access_token: &str,
    ) -> Result<u64, DiscordError> {
        #[serde_as]
        #[derive(Deserialize)]
        struct Payload {
            #[serde_as(as = "DisplayFromStr")]
            id: u64,
        }

        client
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
    async fn fetch_member_info(
        &self,
        user_id: u64,
        client: &reqwest::Client,
        db: &Database,
    ) -> Result<member::Member, DiscordError> {
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
        struct MemberPayload {
            user: UserPayload,
            nick: Option<String>,
            avatar: Option<String>,
            #[serde_as(as = "Vec<DisplayFromStr>")]
            roles: Vec<u64>,
            joined_at: DateTime<Utc>,
        }

        let payload = client
            .get(format!(
                "https://discord.com/api/v10/guilds/{}/members/{}",
                self.guild_id, user_id
            ))
            .header("Authorization", format!("Bot {}", self.token))
            .send()
            .await?
            .json::<MemberPayload>()
            .await?;

        member::insert_member(
            db,
            member::InsertMemberInput {
                id: payload.user.id,
                global_name: payload.user.global_name,
                guild_name: payload.nick,
                global_avatar: payload.user.avatar,
                guild_avatar: payload.avatar,
                joined_at: payload.joined_at,
                roles: payload.roles,
            },
        )
        .await
        .map_err(Into::into)
    }

    pub async fn auth(
        &self,
        code: &str,
        client: &reqwest::Client,
        db: &Database,
    ) -> Result<member::Member, DiscordError> {
        let access_token = self.fetch_user_tokens(client, code).await?;
        let user_id = self.fetch_user_id(client, &access_token).await?;
        self.fetch_member_info(user_id, client, db).await
    }

    pub fn avatar_url(&self, member: &member::Member) -> String {
        if let Some(avatar) = &member.guild_avatar {
            format!(
                "https://cdn.discordapp.com/guilds/{}/users/{}/avatars/{}.png",
                self.guild_id, member.id, avatar
            )
        } else if let Some(avatar) = &member.global_avatar {
            format!(
                "https://cdn.discordapp.com/avatars/{}/{}.png",
                member.id, avatar
            )
        } else {
            let index = (member.id >> 22) % 6;
            format!("https://cdn.discordapp.com/embed/avatars/{}.png", index)
        }
    }
}
