use std::cmp::Ordering;

use super::Error;

#[derive(Debug, Clone)]
pub struct DutyCategory {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct DutyCategoryWithDutiesReview {
    pub id: String,
    pub name: String,
    pub duties: Vec<DutyReview>,
}

#[derive(Debug, Clone)]
pub struct DutyReview {
    pub id: String,
    pub name: String,
    pub short_name: String,
}

#[derive(Debug, Clone)]
pub struct Duty {
    pub id: String,
    pub name: String,
    pub short_name: String,
    pub image_url: String,

    pub phases: Vec<Phase>,
}

#[derive(Debug, Clone)]
pub struct Phase {
    pub name: String,
    // Float, because we can insert a phase between two phases
    pub progression: f64,
}

impl PartialEq for Phase {
    fn eq(&self, other: &Self) -> bool {
        self.progression == other.progression
    }
}
impl Eq for Phase {}

impl PartialOrd for Phase {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.progression.partial_cmp(&other.progression)
    }
}
impl Ord for Phase {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub trait DutyRepo {
    /// Find all duties
    async fn find_all(&self) -> Result<Vec<DutyCategoryWithDutiesReview>, Error>;

    /// Find a duty by its id
    async fn find_by_id(&self, id: &str) -> Result<Duty, Error>;
}
