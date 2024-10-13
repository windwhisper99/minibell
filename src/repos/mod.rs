pub mod discord;
pub mod member;
pub mod session;

#[derive(Debug)]
pub enum RepoError {
    NotFound,
    /// Failed to return from insert
    FailedToInsert,
    ReqwestError(reqwest::Error),
    DatabaseError(surrealdb::Error),
}

impl From<reqwest::Error> for RepoError {
    fn from(value: reqwest::Error) -> Self {
        RepoError::ReqwestError(value)
    }
}

impl From<surrealdb::Error> for RepoError {
    fn from(value: surrealdb::Error) -> Self {
        RepoError::DatabaseError(value)
    }
}
