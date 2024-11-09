use std::sync::Arc;

use async_trait::async_trait;
use aws_config::{BehaviorVersion, SdkConfig};
use minibell::{member, Error};
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};
use shaku::{module, Component};

mod discord;
mod session_hmac;

#[derive(Debug, Clone)]
pub(crate) struct Parameters {
    discord_client_id: String,
    discord_client_secret: String,
    discord_guild_id: u64,
    discord_token: String,

    session_secret: String,
}

#[derive(Debug, Clone)]
pub enum InfraError {
    DependencyError(String),
}

#[derive(Debug, Component)]
#[shaku(interface = member::MemberRepository)]
struct MemberRepoImpl;

#[async_trait]
impl member::MemberRepository for MemberRepoImpl {
    async fn insert_member_and_session(
        &self,
        member: &member::Member,
        session: &member::MemberSession,
    ) -> Result<(), Error> {
        println!("Insert member and session: {:?}", member);
        println!("Insert member and session: {:?}", session);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct BootstrapConfig {
    pub secret_manager_key: Option<String>,
}

module! {
    pub InfraModule {
        components = [
            discord::DiscordClientImpl,
            session_hmac::SessionHmac,
            MemberRepoImpl,
        ],
        providers = [],
    }
}

fn get_env_dotenv() -> Parameters {
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    let discord_client_id =
        std::env::var("DISCORD_CLIENT_ID").expect("DISCORD_CLIENT_ID must be set");
    let discord_client_secret =
        std::env::var("DISCORD_CLIENT_SECRET").expect("DISCORD_CLIENT_SECRET must be set");
    let discord_guild_id = std::env::var("DISCORD_GUILD_ID")
        .expect("DISCORD_GUILD_ID must be set")
        .parse::<u64>()
        .expect("DISCORD_GUILD_ID must be a number");
    let discord_token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set");

    let session_secret = std::env::var("SESSION_SECRET").expect("SESSION_SECRET must be set");

    Parameters {
        discord_client_id,
        discord_client_secret,
        discord_guild_id,
        discord_token,

        session_secret,
    }
}

async fn get_secret_manager(config: &SdkConfig, key: &str) -> Parameters {
    #[serde_as]
    #[derive(Debug, Deserialize)]
    struct Secret {
        discord_client_id: String,
        discord_client_secret: String,
        #[serde_as(as = "DisplayFromStr")]
        discord_guild_id: u64,
        discord_token: String,

        session_secret: String,
    }

    let asm = aws_sdk_secretsmanager::Client::new(config);
    let response = asm
        .get_secret_value()
        .secret_id(key)
        .send()
        .await
        .expect("Failed to get secret");
    let secret = response.secret_string.expect("Secret not found");
    let secret = serde_json::from_str::<Secret>(&secret).expect("Failed to parse secret");

    Parameters {
        discord_client_id: secret.discord_client_id,
        discord_client_secret: secret.discord_client_secret,
        discord_guild_id: secret.discord_guild_id,
        discord_token: secret.discord_token,

        session_secret: secret.session_secret,
    }
}

pub async fn bootstrap(config: BootstrapConfig) -> Result<InfraModule, InfraError> {
    let sdkconfig = aws_config::load_defaults(BehaviorVersion::v2024_03_28()).await;

    let parameters = match config.secret_manager_key {
        Some(key) => get_secret_manager(&sdkconfig, &key).await,
        None => get_env_dotenv(),
    };

    let reqwest = Arc::new(reqwest::Client::new());
    let infra = InfraModule::builder()
        .with_component_parameters::<discord::DiscordClientImpl>(
            discord::DiscordClientImplParameters {
                reqwest,

                client_id: parameters.discord_client_id,
                client_secret: parameters.discord_client_secret,
                guild_id: parameters.discord_guild_id,
                token: parameters.discord_token,
            },
        )
        .with_component_parameters::<session_hmac::SessionHmac>(
            session_hmac::SessionHmacParameters {
                secret: parameters.session_secret,
            },
        )
        .build();

    Ok(infra)
}
