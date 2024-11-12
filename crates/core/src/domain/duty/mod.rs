mod repo;

pub use repo::*;

#[derive(Debug, Clone)]
pub struct DutyCategory {
    pub id: String,
    pub name: String,
    pub parent: Option<String>,
    pub sort: i32,
}

#[derive(Debug, Clone)]
pub struct Duty {
    pub id: String,
    pub category: String,

    pub name: String,
    pub description: Option<String>,
    pub short_name: Option<String>,
    pub patch: f64,
    pub image: String,
    pub sort: i32,
}

#[derive(Debug, Clone)]
pub struct DutyPhrase {
    pub name: String,
    pub progression: f64,
}
