use async_trait::async_trait;

use crate::{
    duty::{CategoriesAndDuties, Duty, DutyCategory, DutyRepository},
    Error,
};

use super::UseCase;

/// Get all duties and categories given a category
pub struct GetDuties<'a> {
    pub duty_repo: &'a dyn DutyRepository,
}

#[derive(Debug, Clone)]
pub struct Input {
    pub category: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub breadcrumbs: Vec<DutyCategory>,
    pub duties: Vec<Duty>,
    pub categories: Vec<DutyCategory>,
}

impl<'a> GetDuties<'a> {
    async fn run(&self, input: Input) -> Result<Response, Error> {
        if let Some(category) = input.category {
            let CategoriesAndDuties {
                breadcrumbs,
                categories,
                duties,
            } = self.duty_repo.list_categories_and_duties(&category).await?;
            Ok(Response {
                breadcrumbs,
                categories,
                duties,
            })
        } else {
            let categories = self.duty_repo.list_categories(None).await?;
            Ok(Response {
                breadcrumbs: vec![],
                categories,
                duties: vec![],
            })
        }
    }
}

#[async_trait]
impl<'a> UseCase for GetDuties<'a> {
    type Input = Input;
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
