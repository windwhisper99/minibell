use std::sync::Arc;

use crate::repos::{
    discord::DiscordRepo,
    member::{self, MemberRepo},
    session::SessionRepo,
};

use super::ServiceError;

#[derive(Clone)]
pub struct AuthService {
    discord: Arc<DiscordRepo>,
    member: Arc<MemberRepo>,
    session: Arc<SessionRepo>,
}

impl AuthService {
    pub fn new(
        discord: Arc<DiscordRepo>,
        member: Arc<MemberRepo>,
        session: Arc<SessionRepo>,
    ) -> Self {
        Self {
            discord,
            member,
            session,
        }
    }

    pub async fn auth(&self, discord_code: &str) -> Result<String, ServiceError> {
        let member_info = self.discord.auth(&discord_code).await?;
        let member = self
            .member
            .insert(member::InsertInput {
                id: member_info.id,
                name: member_info.name,
                avatar: member_info.avatar,
                joined_at: member_info.joined_at,
                roles: member_info.roles,
            })
            .await?;

        self.session
            .new_session(member.id)
            .await
            .map_err(Into::into)
    }
}
