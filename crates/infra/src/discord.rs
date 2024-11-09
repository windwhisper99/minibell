use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};
use shaku::Component;

use minibell::{member, Error};

/// Discord request client service
#[derive(Clone, Component)]
#[shaku(interface = member::DiscordClient)]
pub struct DiscordClientImpl {
    reqwest: Arc<reqwest::Client>,

    client_id: String,
    client_secret: String,
    guild_id: u64,
    token: String,
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
    joined_at: DateTime<Utc>,
}

fn reqwest_error_to_error(err: reqwest::Error) -> Error {
    Error::internal(err.to_string())
}

impl DiscordClientImpl {
    /// Fetch access token from Oauth2 code
    async fn fetch_user_tokens(&self, code: &str, redirect_uri: &str) -> Result<String, Error> {
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
                ("redirect_uri", redirect_uri),
            ])
            .send()
            .await
            .map_err(reqwest_error_to_error)?
            .json::<AuthorizationResult>()
            .await
            .map(|payload| payload.access_token)
            .map_err(reqwest_error_to_error)
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
            .await
            .map_err(reqwest_error_to_error)?
            .json::<Payload>()
            .await
            .map(|payload| payload.id)
            .map_err(reqwest_error_to_error)
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
            .await
            .map_err(reqwest_error_to_error)?
            .json::<MemberPayload>()
            .await
            .map_err(reqwest_error_to_error)
    }

    async fn auth(&self, code: &str, redirect_uri: &str) -> Result<member::Member, Error> {
        let access_token = self.fetch_user_tokens(code, redirect_uri).await?;
        let user_id = self.fetch_user_id(&access_token).await?;
        let payload = self.fetch_member_info(user_id).await?;
        let avatar_url = self.avatar_url(&payload);

        Ok(member::Member::new(
            payload.user.id,
            payload
                .nick
                .clone()
                .unwrap_or(payload.user.global_name.clone()),
            avatar_url,
            payload.joined_at,
        ))
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

    fn get_oauth2_url(&self, redirect_uri: &str) -> String {
        format!(
            "https://discord.com/api/oauth2/authorize?client_id={}&redirect_uri={}&response_type=code&scope=identify",
            self.client_id, url_escape::encode_www_form_urlencoded(redirect_uri)
        )
    }
}

#[async_trait]
impl member::DiscordClient for DiscordClientImpl {
    async fn sign_in(&self, code: &str, redirect_uri: &str) -> Result<member::Member, Error> {
        self.auth(code, redirect_uri).await
    }

    async fn get_member(&self, member_id: member::MemberId) -> Result<member::Member, Error> {
        self.get_member(member_id).await
    }

    fn get_oauth2_url(&self, redirect_uri: &str) -> String {
        self.get_oauth2_url(redirect_uri)
    }
}
