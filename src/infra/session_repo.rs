use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::Deserialize;
use surrealdb::RecordId;

use crate::domain::{auth, member::Member, Error};

use super::{db::Database, MemberModel};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(super) enum MemberIDModel {
    Id(u64),
    Model(MemberModel),
}

#[derive(Debug, Deserialize)]
pub(super) struct SessionModel {
    pub id: String,
    pub member: MemberIDModel,
    pub issued_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl TryInto<auth::Session> for SessionModel {
    type Error = Error;

    fn try_into(self) -> Result<auth::Session, Self::Error> {
        Ok(auth::Session {
            id: self.id,
            member_id: match self.member {
                MemberIDModel::Id(id) => id,
                MemberIDModel::Model(model) => model.id,
            },
            issued_at: self.issued_at,
            updated_at: self.updated_at,
            expires_at: self.expires_at,
        })
    }
}

impl TryInto<auth::SessionWithMember> for SessionModel {
    type Error = Error;

    fn try_into(self) -> Result<auth::SessionWithMember, Self::Error> {
        let member: Member = match self.member {
            MemberIDModel::Model(member) => member.into(),
            _ => return Err(Error::FailedToDecode),
        };

        Ok(auth::SessionWithMember {
            session: auth::Session {
                id: self.id,
                member_id: member.id,
                issued_at: self.issued_at,
                updated_at: self.updated_at,
                expires_at: self.expires_at,
            },
            member,
        })
    }
}

#[derive(Clone)]
pub struct SessionRepo {
    db: Arc<Database>,
}

impl SessionRepo {
    pub fn new(db: Arc<Database>) -> Self {
        SessionRepo { db }
    }
}

impl auth::SessionRepo for &SessionRepo {
    async fn create<I>(&self, input: I) -> Result<auth::Session, Error>
    where
        I: Into<auth::SessionCreateInput>,
    {
        let input: auth::SessionCreateInput = input.into();

        self.db
            .query(
                "CREATE ONLY session SET 
                    member = $member,
                    issued_at = <datetime>$ist,
                    updated_at = <datetime>$upat,
                    expires_at = <datetime>$exp
                RETURN *, id.id(), member.id()",
            )
            .bind((
                "member",
                RecordId::from_table_key("member", input.member_id as i64),
            ))
            .bind(("ist", input.issued_at))
            .bind(("upat", input.updated_at))
            .bind(("exp", input.expires_at))
            .await?
            .take::<Option<SessionModel>>(0)?
            .ok_or_else(|| Error::FailedToInsert)
            .and_then(TryInto::try_into)
    }

    async fn get(&self, session_id: &str) -> Result<auth::SessionWithMember, Error> {
        self.db
            .query(
                "SELECT *,
                    id.id(),
                    member.{
                        name,
                        avatar,
                        joined_at,
                        updated_at,
                        roles,
                        id: id.id()
                    }
                FROM ONLY session 
                WHERE id = $id LIMIT 1 FETCH member",
            )
            .bind(("id", RecordId::from_table_key("session", session_id)))
            .await?
            .take::<Option<SessionModel>>(0)?
            .ok_or_else(|| Error::NotFound)
            .and_then(TryInto::try_into)
    }
}
