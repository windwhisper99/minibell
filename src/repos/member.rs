use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::utils::db::Database;

use super::RepoError;

#[derive(Debug, Deserialize, Serialize)]
pub struct Member {
    pub id: u64,
    pub name: String,
    pub avatar: String,
    pub roles: Vec<u64>,
}

#[derive(Debug, Serialize)]
pub struct InsertInput {
    pub id: u64,
    pub name: String,
    pub avatar: String,
    pub joined_at: DateTime<Utc>,
    pub roles: Vec<u64>,
}

#[derive(Clone)]
pub struct MemberRepo {
    db: Arc<Database>,
}

impl MemberRepo {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn insert(&self, input: InsertInput) -> Result<Member, RepoError> {
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
                name = $input.name,
                avatar = $input.avatar,
                joined_at = <datetime>$input.joined_at,
                updated_at = time::now(),
                roles = $input.roles
            RETURN *, id.id()",
            )
            .bind(input)
            .await?
            .check()?
            .take::<Option<Member>>(0)?
            .ok_or_else(|| RepoError::NotFound)
    }
}
