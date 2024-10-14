use crate::domain::{
    auth::{AccessType, SessionRepo, SessionToken, SessionTokenService},
    Error,
};

pub struct VerifyUC<SessionR, SessionTokenS> {
    pub session_repo: SessionR,

    pub session_token_service: SessionTokenS,
}

impl<SessionR, SessionTokenS> VerifyUC<SessionR, SessionTokenS>
where
    SessionR: SessionRepo,
    SessionTokenS: SessionTokenService,
{
    pub fn new(session_repo: SessionR, session_token_service: SessionTokenS) -> Self {
        Self {
            session_repo,
            session_token_service,
        }
    }

    /// Execute verify access token
    pub async fn execute(&self, token: &str) -> Result<AccessType, Error> {
        let token = match self.session_token_service.verify_token(token).await {
            Ok(token) => token,
            Err(Error::Unauthenticated) => return Ok(AccessType::Unauthenticated),
            Err(err) => return Err(err),
        };
        let session = match self.session_repo.get(token.session_id()).await {
            Ok(session) => session,
            Err(Error::NotFound) => return Ok(AccessType::Unauthenticated),
            Err(err) => return Err(err),
        };

        Ok(AccessType::Session(session))
    }
}
