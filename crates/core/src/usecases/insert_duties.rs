use async_trait::async_trait;

use crate::{
    duty::{Duty, DutyPhrase, DutyRepository},
    Error,
};

use super::UseCase;

pub struct InsertDuties<'a> {
    pub duty_repo: &'a dyn DutyRepository,
}

#[derive(Debug, Clone)]
pub struct Input {
    pub duties: Vec<(Duty, Vec<DutyPhrase>)>,
}

#[async_trait]
impl<'a> UseCase for InsertDuties<'a> {
    type Input = Input;
    type Response = ();

    async fn system_execute(&self, input: Self::Input) -> Result<Self::Response, Error> {
        if input.duties.is_empty() {
            return Ok(());
        }

        self.duty_repo
            .insert_duties(
                &input
                    .duties
                    .iter()
                    .map(|(duty, pharses)| (duty.clone(), pharses.as_slice()))
                    .collect::<Vec<_>>(),
            )
            .await
    }
}
