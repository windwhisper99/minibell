use async_trait::async_trait;

use crate::{
    duty::{Duty, DutyPhrase, DutyRepository},
    Error,
};

use super::UseCase;

pub struct GetDuty<'a> {
    pub duty_repo: &'a dyn DutyRepository,
}

impl<'a> GetDuty<'a> {
    async fn run(&self, id: &str) -> Result<(Duty, Vec<DutyPhrase>), Error> {
        self.duty_repo.get_duty(id).await
    }
}

#[async_trait]
impl<'a> UseCase for GetDuty<'a> {
    type Input = &'a str;
    type Response = (Duty, Vec<DutyPhrase>);

    async fn guest_execute(&self, input: Self::Input) -> Result<Self::Response, Error> {
        self.run(input).await
    }

    async fn member_execute(
        &self,
        _member_id: crate::member::MemberId,
        input: Self::Input,
    ) -> Result<Self::Response, Error> {
        self.run(input).await
    }
}
