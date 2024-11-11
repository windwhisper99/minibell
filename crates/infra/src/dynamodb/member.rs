use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use minibell::{
    member::{self, MemberId},
    Error,
};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, TimestampMilliSeconds};
use shaku::Component;

use super::{DynamoClient, PrimaryModel};

#[derive(Debug, Component)]
#[shaku(interface = member::MemberRepository)]
pub struct MemberRepoImpl {
    db: Arc<DynamoClient>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
struct MemberModel {
    id: u64,
    name: String,
    avatar: String,

    #[serde_as(as = "TimestampMilliSeconds")]
    updated_at: DateTime<Utc>,
    #[serde_as(as = "TimestampMilliSeconds")]
    joined_at: DateTime<Utc>,
}

impl From<&member::Member> for MemberModel {
    fn from(member: &member::Member) -> Self {
        Self {
            id: member.id,
            name: member.display_name.clone(),
            avatar: member.avatar.clone(),

            updated_at: member.updated_at,
            joined_at: member.joined_at,
        }
    }
}

impl Into<member::Member> for MemberModel {
    fn into(self) -> member::Member {
        member::Member {
            id: self.id,
            display_name: self.name,
            avatar: self.avatar,

            updated_at: self.updated_at,
            joined_at: self.joined_at,
        }
    }
}

impl PrimaryModel for MemberModel {
    fn data_type(&self) -> String {
        "Member".to_string()
    }

    fn primary_key(&self) -> String {
        "MEMBER".to_string()
    }

    fn sort_key(&self) -> String {
        format!("MEMBER#{}", self.id)
    }

    fn gsi1(&self) -> Option<(String, String)> {
        Some((
            "MEMBER".to_string(),
            format!("MEMBER#{}", self.updated_at.timestamp_millis()),
        ))
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
struct MemberSessionModel {
    id: String,
    member_id: u64,

    #[serde_as(as = "TimestampMilliSeconds")]
    issued_at: DateTime<Utc>,
    #[serde_as(as = "TimestampMilliSeconds")]
    expires_at: DateTime<Utc>,
}

impl PrimaryModel for MemberSessionModel {
    fn data_type(&self) -> String {
        "MemberSession".to_string()
    }

    fn primary_key(&self) -> String {
        "MEMBER_SESSION".to_string()
    }

    fn sort_key(&self) -> String {
        format!("MEMBER_SESSION#{}", self.id)
    }

    fn gsi1(&self) -> Option<(String, String)> {
        Some((
            format!("MEMBER#{}", self.member_id),
            format!("MEMBER_SESSION#{}", self.id),
        ))
    }

    fn gsi2(&self) -> Option<(String, String)> {
        Some((
            "MEMBER_SESSION".to_string(),
            format!("MEMBER_SESSION#{}", self.expires_at.timestamp_millis()),
        ))
    }
}

impl From<&member::MemberSession> for MemberSessionModel {
    fn from(session: &member::MemberSession) -> Self {
        Self {
            id: session.id.clone(),
            member_id: session.member_id,

            issued_at: session.issued_at,
            expires_at: session.expires_at,
        }
    }
}

impl Into<member::MemberSession> for MemberSessionModel {
    fn into(self) -> member::MemberSession {
        member::MemberSession {
            id: self.id,
            member_id: self.member_id,

            issued_at: self.issued_at,
            expires_at: self.expires_at,
        }
    }
}

#[async_trait]
impl member::MemberRepository for MemberRepoImpl {
    async fn insert_member_and_session(
        &self,
        member: &member::Member,
        session: &member::MemberSession,
    ) -> Result<(), Error> {
        self.db
            .batch_insert_items()
            .add_item(MemberModel::from(member))?
            .add_item(MemberSessionModel::from(session))?
            .send()
            .await
    }

    async fn get_member(&self, member_id: MemberId) -> Result<member::Member, Error> {
        self.db
            .get_item::<MemberModel>("MEMBER", &format!("MEMBER#{}", member_id))
            .await
            .map(Into::into)
    }

    async fn get_member_session(&self, session_id: &str) -> Result<member::MemberSession, Error> {
        self.db
            .get_item::<MemberSessionModel>(
                "MEMBER_SESSION",
                &format!("MEMBER_SESSION#{}", session_id),
            )
            .await
            .map(Into::into)
    }
}
