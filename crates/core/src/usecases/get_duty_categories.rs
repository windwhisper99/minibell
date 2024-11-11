use async_trait::async_trait;

use crate::{
    duty::{DutyCategory, DutyRepository},
    Error,
};

use super::UseCase;

pub struct GetDutyCategories<'a> {
    pub duty_repo: &'a dyn DutyRepository,
}

#[derive(Debug, Clone)]
pub struct Input {
    pub parent: Option<String>,
}

impl<'a> GetDutyCategories<'a> {
    async fn run(&self, input: Input) -> Result<Vec<DutyCategory>, Error> {
        self.duty_repo
            .list_categories(input.parent.as_deref())
            .await
    }
}

#[async_trait]
impl<'a> UseCase for GetDutyCategories<'a> {
    type Input = Input;
    type Response = Vec<DutyCategory>;

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
