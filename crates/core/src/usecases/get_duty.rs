use async_trait::async_trait;

use crate::{
    duty::{Duty, DutyCategory, DutyPhrase, DutyRepository},
    Error,
};

use super::UseCase;

/// Get duty and breadcrumbs categories
pub struct GetDuty<'a> {
    pub duty_repo: &'a dyn DutyRepository,
}

#[derive(Debug)]
pub struct Response {
    pub breadcrumbs: Vec<DutyCategory>,
    pub duty: Duty,
    pub phrases: Vec<DutyPhrase>,
}

impl<'a> GetDuty<'a> {
    async fn run(&self, id: &str) -> Result<Response, Error> {
        self.duty_repo.get_duty(id).await.map(|d| Response {
            breadcrumbs: d.breadcrumbs,
            duty: d.duty,
            phrases: d.phrases,
        })
    }
}

#[async_trait]
impl<'a> UseCase for GetDuty<'a> {
    type Input = &'a str;
    type Response = Response;

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
