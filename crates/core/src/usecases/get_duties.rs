use async_trait::async_trait;

use crate::{
    duty::{Duty, DutyRepository},
    Error,
};

use super::UseCase;

pub struct GetDuties<'a> {
    pub duty_repo: &'a dyn DutyRepository,
}

#[derive(Debug, Clone)]
pub struct Input {
    pub category: String,
}

impl<'a> GetDuties<'a> {
    async fn run(&self, input: Input) -> Result<Vec<Duty>, Error> {
        self.duty_repo.list_duties(&input.category).await
    }
}

#[async_trait]
impl<'a> UseCase for GetDuties<'a> {
    type Input = Input;
    type Response = Vec<Duty>;

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
