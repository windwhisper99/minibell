use async_trait::async_trait;
use shaku::Interface;

use crate::Error;

use super::{Duty, DutyCategory, DutyPhrase};

#[async_trait]
pub trait DutyRepository: Interface {
    /// Insert a category
    /// Create or update if exists
    async fn insert_categories(&self, categories: &[DutyCategory]) -> Result<(), Error>;
    /// Insert a duty
    /// Create or update if exists
    async fn insert_duties(&self, duties: &[(Duty, &[DutyPhrase])]) -> Result<(), Error>;

    /// List all categories or list categories by parent
    async fn list_categories(&self, parent: Option<&str>) -> Result<Vec<DutyCategory>, Error>;
    /// List all duties given category
    async fn list_duties(&self, category: &str) -> Result<Vec<Duty>, Error>;

    /// Get a duty will pharse
    async fn get_duty(&self, duty_id: &str) -> Result<(Duty, Vec<DutyPhrase>), Error>;
}
