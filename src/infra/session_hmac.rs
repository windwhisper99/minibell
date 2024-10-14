use std::{env, str::FromStr};

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use hmac::Mac;
use sha2::Sha256;

use crate::domain::{auth, Error};

#[derive(Clone)]
pub struct SessionHmac {
    secret: Vec<u8>,
}

#[derive(Debug)]
pub struct SessionToken {
    session_id: String,
    signature: String,
}

impl ToString for SessionToken {
    fn to_string(&self) -> String {
        format!("{}.{}", self.session_id, self.signature)
    }
}

impl auth::SessionToken for SessionToken {
    fn session_id(&self) -> &str {
        &self.session_id
    }
}

impl FromStr for SessionToken {
    type Err = SessionHmacError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 2 {
            return Err(SessionHmacError::InvalidToken);
        }

        Ok(SessionToken {
            session_id: parts[0].to_string(),
            signature: parts[1].to_string(),
        })
    }
}

#[derive(Debug)]
pub enum SessionHmacError {
    InvalidSecret,
    InvalidToken,
}

impl From<SessionHmacError> for Error {
    fn from(err: SessionHmacError) -> Self {
        match err {
            SessionHmacError::InvalidSecret => Error::ServiceError("Invalid secret key".into()),
            SessionHmacError::InvalidToken => Error::Unauthenticated,
        }
    }
}

impl SessionHmac {
    pub fn new() -> Self {
        let secret = env::var("SESSION_SECRET")
            .expect("SESSION_SECRET much be set")
            .as_bytes()
            .to_vec();

        SessionHmac { secret }
    }

    fn sign_signature(&self, session_id: &str) -> Result<String, SessionHmacError> {
        type HmacSha256 = hmac::Hmac<Sha256>;

        let mut mac = HmacSha256::new_from_slice(&self.secret)
            .map_err(|_| SessionHmacError::InvalidSecret)?;
        mac.update(session_id.as_bytes());

        Ok(URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes()))
    }

    fn verify_signature(&self, session_id: &str, token: &str) -> Result<(), SessionHmacError> {
        self.sign_signature(session_id).and_then(|signature| {
            if signature == token {
                Ok(())
            } else {
                Err(SessionHmacError::InvalidToken)
            }
        })
    }
}

impl auth::SessionTokenService for &SessionHmac {
    type SessionToken = SessionToken;

    async fn sign_session(&self, session_id: &str) -> Result<Self::SessionToken, Error> {
        let signature = self.sign_signature(session_id)?;
        Ok(SessionToken {
            session_id: session_id.to_string(),
            signature,
        })
    }

    async fn verify_token(&self, token: &str) -> Result<Self::SessionToken, Error> {
        let signuture = token.parse::<SessionToken>()?;
        self.verify_signature(&signuture.session_id, &signuture.signature)?;
        Ok(signuture)
    }
}
