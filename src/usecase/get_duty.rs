use crate::domain::{
    duty::{Duty, DutyRepo},
    Error,
};

/// Get single duty by id
pub struct GetDuty<DutyRepo> {
    pub duty_repo: DutyRepo,
}

impl<DutyR> GetDuty<DutyR>
where
    DutyR: DutyRepo,
{
    pub fn new(duty_repo: DutyR) -> Self {
        Self { duty_repo }
    }

    /// Get list of duties, Include all categories
    pub async fn execute(&self, id: &str) -> Result<Duty, Error> {
        let duties = self.duty_repo.find_by_id(id).await?;
        Ok(duties)
    }
}
