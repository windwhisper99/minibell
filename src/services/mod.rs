use crate::repos::RepoError;

pub mod auth;

#[derive(Debug)]
pub enum ServiceError {
    RepoError(RepoError),
}

impl From<RepoError> for ServiceError {
    fn from(value: RepoError) -> Self {
        ServiceError::RepoError(value)
    }
}
