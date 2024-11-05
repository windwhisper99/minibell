pub mod auth;
pub mod duty;
pub mod event;
pub mod member;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("internal error: {0}")]
    Internal(String),
    #[error("response error: {0}")]
    RepositoryError(String),
    #[error("service error: {0}")]
    ServiceError(String),

    #[error("unauthenticated")]
    Unauthenticated,
    #[error("forbidden")]
    Forbidden,
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("bad automation")]
    BadAutomation,
    #[error("record already exists")]
    AlreadyExists,

    /// Failed to insert the record
    #[error("failed to insert the record")]
    FailedToInsert,
    /// Failed to decode result
    #[error("failed to decode result")]
    FailedToDecode,
    /// Pesistence repository cannot find the record
    #[error("record not found")]
    NotFound,
}
