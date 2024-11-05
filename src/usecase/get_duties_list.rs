use crate::domain::{
    duty::{DutyCategoryWithDutiesReview, DutyRepo},
    Error,
};

/// Get list of duties
pub struct GetDutiesList<DutyRepo> {
    pub duty_repo: DutyRepo,
}

impl<DutyR> GetDutiesList<DutyR>
where
    DutyR: DutyRepo,
{
    pub fn new(duty_repo: DutyR) -> Self {
        Self { duty_repo }
    }

    /// Get list of duties, Include all categories
    pub async fn execute(&self) -> Result<Vec<DutyCategoryWithDutiesReview>, Error> {
        let duties = self.duty_repo.find_all().await?;
        Ok(duties)
    }
}
