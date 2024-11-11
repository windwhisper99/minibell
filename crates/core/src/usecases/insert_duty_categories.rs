use async_trait::async_trait;

use crate::{
    duty::{DutyCategory, DutyRepository},
    Error,
};

use super::UseCase;

pub struct InsertDutyCategories<'a> {
    pub duty_repo: &'a dyn DutyRepository,
}

#[derive(Debug, Clone)]
pub struct Input {
    pub categories: Vec<DutyCategory>,
}

#[async_trait]
impl<'a> UseCase for InsertDutyCategories<'a> {
    type Input = Input;
    type Response = ();

    async fn system_execute(&self, input: Self::Input) -> Result<Self::Response, Error> {
        if input.categories.is_empty() {
            Ok(())
        } else {
            self.duty_repo.insert_categories(&input.categories).await
        }
    }
}
