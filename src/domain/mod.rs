pub mod auth;
pub mod member;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("response error: {0}")]
    RepositoryError(String),
    #[error("service error: {0}")]
    ServiceError(String),

    #[error("unauthenticated")]
    Unauthenticated,

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
