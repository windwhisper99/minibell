use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};
use chrono::{DateTime, Utc};
use rand::{rngs::OsRng, RngCore};
use sha2::{Digest, Sha256};

/// Sign in session
#[derive(Debug, Clone)]
pub struct MemberSession {
    pub id: String,
    pub member_id: u64,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl MemberSession {
    /// Generate a new session id
    fn gen_id() -> (String, DateTime<Utc>) {
        let now = Utc::now();
        let mut rand_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut rand_bytes);

        let mut hasher = Sha256::new();
        hasher.update(&rand_bytes);
        hasher.update(now.timestamp_millis().to_be_bytes());
        let hash = hasher.finalize();

        (BASE64_URL_SAFE_NO_PAD.encode(&hash), now)
    }

    pub fn new(member_id: u64, duration: chrono::Duration) -> Self {
        let (id, now) = Self::gen_id();

        Self {
            id,
            member_id,
            issued_at: now,
            expires_at: now + duration,
        }
    }

    // fn sign_signature(session_id: &str, secret: &str) -> Result<String, Error> {
    //     type HmacSha256 = hmac::Hmac<Sha256>;

    //     let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
    //         .map_err(|_| Error::internal("Sign signature failed, secret is invalid."))?;
    //     mac.update(session_id.as_bytes());

    //     Ok(URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes()))
    // }

    // fn verify_signature(session_id: &str, token: &str, secret: &str) -> Result<(), Error> {
    //     Self::sign_signature(session_id, secret).and_then(|signature| {
    //         if signature == token {
    //             Ok(())
    //         } else {
    //             Err(Error::InvalidToken)
    //         }
    //     })
    // }

    // pub fn tokenize<S: MemberSessionSigner>(&self, signer: S) -> Result<String, Error> {
    //     let signature = signer.sign_signature(&self.id)?;
    //     Ok(format!("{}.{}", self.id, signature))
    // }

    // /// Verify the token, and return the session id
    // pub fn verify<S: MemberSessionSigner>(token: &str, signer: S) -> Result<String, Error> {
    //     let parts: Vec<&str> = token.split('.').collect();
    //     if parts.len() != 2 {
    //         return Err(Error::InvalidToken);
    //     }

    //     let session_id = parts[0];
    //     let signature = parts[1];

    //     signer.verify_signature(session_id, signature)?;
    //     Ok(session_id.to_string())
    // }
}
