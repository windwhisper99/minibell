use async_trait::async_trait;

use crate::{
    member::{MemberRepository, MemberSession, MemberSessionSigner},
    Error,
};

use super::UseCase;

pub struct Authorization<'a> {
    pub member_repo: &'a dyn MemberRepository,
    pub member_session_signer: &'a dyn MemberSessionSigner,
}

impl<'a> Authorization<'a> {
    async fn run(&self, token: &str) -> Result<Option<MemberSession>, Error> {
        let session_id = self.member_session_signer.verify(token)?;
        let session = match self.member_repo.get_member_session(&session_id).await {
            Ok(session) => Some(session),
            Err(Error::ItemNotFound) => None,
            Err(e) => return Err(e),
        };

        Ok(session)
    }
}

#[async_trait]
impl<'a> UseCase for Authorization<'a> {
    type Input = &'a str;
    type Response = Option<MemberSession>;

    async fn guest_execute(&self, token: Self::Input) -> Result<Self::Response, Error> {
        self.run(token).await
    }
}
