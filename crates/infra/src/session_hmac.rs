use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};
use hmac::Mac;
use minibell::{member, Error};
use sha2::Sha256;
use shaku::Component;

#[derive(Debug, Clone, Component)]
#[shaku(interface = member::MemberSessionSigner)]
pub struct SessionHmac {
    secret: String,
}

impl SessionHmac {
    fn sign_signature(&self, session_id: &str) -> Result<String, Error> {
        type HmacSha256 = hmac::Hmac<Sha256>;

        let mut mac = HmacSha256::new_from_slice(self.secret.as_bytes())
            .map_err(|_| Error::internal("Sign signature failed, secret is invalid."))?;
        mac.update(session_id.as_bytes());

        Ok(BASE64_URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes()))
    }
}

impl member::MemberSessionSigner for SessionHmac {
    fn sign(&self, session_id: &str) -> Result<String, Error> {
        let signature = self.sign_signature(session_id)?;
        Ok(format!("{}.{}", session_id, signature))
    }

    fn verify(&self, token: &str) -> Result<String, Error> {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 2 {
            return Err(Error::InvalidToken);
        }

        let session_id = parts[0];
        let signature = parts[1];

        self.sign_signature(session_id).and_then(|com_signature| {
            if com_signature == signature {
                Ok(session_id.to_string())
            } else {
                Err(Error::InvalidToken)
            }
        })
    }
}
