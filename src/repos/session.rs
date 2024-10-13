use std::{env, sync::Arc};

use base64::Engine;
use chrono::{Duration, Utc};
use hmac::Mac;
use sha2::Sha256;
use surrealdb::RecordId;

use crate::utils::db::Database;

use super::RepoError;

#[derive(Clone)]
pub struct SessionRepo {
    db: Arc<Database>,
    secret: Vec<u8>,
}

impl SessionRepo {
    pub fn new(db: Arc<Database>) -> Self {
        let secret = env::var("SESSION_SECRET")
            .expect("SESSION_SECRET much be set")
            .as_bytes()
            .to_vec();

        SessionRepo { db, secret }
    }

    fn sign_signature(&self, session_id: &str) -> String {
        type HmacSha256 = hmac::Hmac<Sha256>;

        let mut mac =
            HmacSha256::new_from_slice(&self.secret).expect("HMAC can take key of any size.");
        mac.update(session_id.as_bytes());

        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes())
    }

    pub async fn new_session(&self, member_id: u64) -> Result<String, RepoError> {
        // Insert new session
        let tt = Utc::now();
        let expires_at = tt + Duration::weeks(1);

        let id = self
            .db
            .query(
                "CREATE ONLY session SET 
                    member = $member,
                    issued_at = <datetime>$tt,
                    updated_at = <datetime>$tt,
                    expires_at = <datetime>$exp
                RETURN
                    id.id()",
            )
            .bind((
                "member",
                RecordId::from_table_key("member", member_id as i64),
            ))
            .bind(("tt", tt))
            .bind(("exp", expires_at))
            .await?
            .take::<Option<String>>((0, "id"))?
            .ok_or_else(|| RepoError::FailedToInsert)?;
        let signature = self.sign_signature(&id);

        Ok(format!("{}.{}", id, signature))
    }
}
