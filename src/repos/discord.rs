use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

use crate::repos::RepoError;

#[derive(Clone)]
pub struct DiscordRepo {
    reqwest: Arc<reqwest::Client>,

    client_id: String,
    client_secret: String,
    guild_id: u64,
    token: String,

    redirect_uri: String,
}

#[derive(Debug, Clone)]
pub struct MemberInfo {
    pub id: u64,
    pub name: String,
    pub avatar: String,
    pub joined_at: DateTime<Utc>,
    pub roles: Vec<u64>,
}

impl DiscordRepo {
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
    pub async fn fetch_user_tokens(&self, code: &str) -> Result<String, RepoError> {
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
    pub async fn fetch_user_id(&self, access_token: &str) -> Result<u64, RepoError> {
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
    pub async fn fetch_member_info(&self, user_id: u64) -> Result<MemberInfo, RepoError> {
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

        let payload = self
            .reqwest
            .get(format!(
                "https://discord.com/api/v10/guilds/{}/members/{}",
                self.guild_id, user_id
            ))
            .header("Authorization", format!("Bot {}", self.token))
            .send()
            .await?
            .json::<MemberPayload>()
            .await?;

        Ok(MemberInfo {
            id: payload.user.id,
            name: payload.nick.unwrap_or(payload.user.global_name),
            avatar: self.avatar_url(payload.user.id, &payload.avatar, &payload.user.avatar),
            joined_at: payload.joined_at,
            roles: payload.roles,
        })
    }

    pub async fn auth(&self, code: &str) -> Result<MemberInfo, RepoError> {
        let access_token = self.fetch_user_tokens(code).await?;
        let user_id = self.fetch_user_id(&access_token).await?;
        self.fetch_member_info(user_id).await
    }

    fn avatar_url(
        &self,
        id: u64,
        guild_avatar: &Option<String>,
        global_avatar: &Option<String>,
    ) -> String {
        if let Some(avatar) = &guild_avatar {
            format!(
                "https://cdn.discordapp.com/guilds/{}/users/{}/avatars/{}.png",
                self.guild_id, id, avatar
            )
        } else if let Some(avatar) = &global_avatar {
            format!("https://cdn.discordapp.com/avatars/{}/{}.png", id, avatar)
        } else {
            let index = (id >> 22) % 6;
            format!("https://cdn.discordapp.com/embed/avatars/{}.png", index)
        }
    }
}
