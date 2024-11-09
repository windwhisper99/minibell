use async_trait::async_trait;

use crate::{member::MemberId, AccessType, Error};

pub mod get_auth_info;
pub mod sign_in;

#[async_trait]
pub trait UseCase: Send {
    type Input: Send;
    type Response: Send;

    async fn system_execute(&self, input: Self::Input) -> Result<Self::Response, Error> {
        let _ = input;
        Err(Error::Forbidden)
    }

    async fn member_execute(
        &self,
        member_id: MemberId,
        input: Self::Input,
    ) -> Result<Self::Response, Error> {
        let _ = input;
        let _ = member_id;
        Err(Error::Forbidden)
    }

    async fn guest_execute(&self, input: Self::Input) -> Result<Self::Response, Error> {
        let _ = input;
        Err(Error::Forbidden)
    }

    async fn execute(
        &self,
        access_type: &AccessType,
        input: Self::Input,
    ) -> Result<Self::Response, Error> {
        match access_type {
            AccessType::System => self.system_execute(input).await,
            AccessType::Member(member_id) => self.member_execute(*member_id, input).await,
            AccessType::Guest => self.guest_execute(input).await,
        }
    }
}
