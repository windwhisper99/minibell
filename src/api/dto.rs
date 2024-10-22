use serde::Serialize;

use crate::domain;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberDto {
    pub id: u64,
    pub name: String,
    pub avatar: String,
}

impl From<domain::member::Member> for MemberDto {
    fn from(member: domain::member::Member) -> Self {
        Self {
            id: member.id,
            name: member.display_name,
            avatar: member.avatar,
        }
    }
}
