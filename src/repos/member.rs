use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::utils::db::Database;

use super::RepoError;

#[derive(Debug, Deserialize, Serialize)]
pub struct Member {
    pub id: u64,
    pub display_name: String,
    pub global_avatar: Option<String>,
    pub guild_avatar: Option<String>,
    pub roles: Vec<u64>,
}

#[derive(Debug, Serialize)]
pub struct InsertMemberInput {
    pub id: u64,
    pub global_name: String,
    pub guild_name: Option<String>,
    pub global_avatar: Option<String>,
    pub guild_avatar: Option<String>,
    pub joined_at: DateTime<Utc>,
    pub roles: Vec<u64>,
}

pub async fn insert_member(db: &Database, input: InsertMemberInput) -> Result<Member, RepoError> {
    db.query(
        "INSERT INTO member {
                id: $id,
                global_name: $global_name,
                global_avatar: $global_avatar,
                guild_name: $guild_name,
                guild_avatar: $guild_avatar,
                joined_at: <datetime>$joined_at,
                roles: $roles
            } 
            ON DUPLICATE KEY UPDATE
                global_name = $input.global_name,
                global_avatar = $input.global_avatar,
                guild_name = $input.guild_name,
                guild_avatar = $input.guild_avatar,
                joined_at = <datetime>$input.joined_at,
                updated_at = time::now(),
                roles = $input.roles
            RETURN 
                id.id(),
                (guild_name || global_name) as display_name,
                global_avatar,
                guild_avatar,
                roles",
    )
    .bind(input)
    .await?
    .check()?
    .take::<Option<Member>>(0)?
    .ok_or_else(|| RepoError::NotFound)
}
