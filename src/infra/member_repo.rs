use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::{member, Error};

use super::db::Database;

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct MemberModel {
    pub id: u64,
    pub name: String,
    pub avatar: String,
    pub roles: Vec<u64>,
    pub joined_at: DateTime<Utc>,

    pub updated_at: DateTime<Utc>,
}

impl Into<member::Member> for MemberModel {
    fn into(self) -> member::Member {
        member::Member {
            id: self.id,
            display_name: self.name,
            avatar: self.avatar,
            roles: self.roles,
            joined_at: self.joined_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Clone)]
pub struct MemberRepo {
    db: Arc<Database>,
}

impl MemberRepo {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }
}

impl member::MemberRepo for &MemberRepo {
    async fn insert<M>(&self, input: M) -> Result<member::Member, Error>
    where
        M: Into<member::MemberInsertInput>,
    {
        let input = input.into();

        #[derive(Debug, Serialize)]
        struct Input {
            id: u64,
            name: String,
            avatar: String,
            joined_at: DateTime<Utc>,
            roles: Vec<u64>,
        }

        self.db
            .query(
                "INSERT INTO member {
                    id: $id,
                    name: $name,
                    avatar: $avatar,
                    joined_at: <datetime>$joined_at,
                    roles: $roles
                }
                ON DUPLICATE KEY UPDATE
                    name = $name,
                    avatar = $avatar,
                    joined_at = <datetime>$joined_at,
                    updated_at = time::now(),
                    roles = $roles
                RETURN *, id.id()",
            )
            .bind(Input {
                id: input.id,
                name: input.display_name,
                avatar: input.avatar,
                joined_at: input.joined_at,
                roles: input.roles,
            })
            .await?
            .check()?
            .take::<Option<MemberModel>>(0)?
            .ok_or_else(|| Error::NotFound)
            .map(Into::into)
    }
}
