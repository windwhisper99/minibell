pub mod auth;
pub mod member;

#[derive(Debug)]
pub enum Error {
    RepositoryError(String),
    ServiceError(String),

    Unauthenticated,

    /// Failed to insert the record
    FailedToInsert,
    /// Failed to decode result
    FailedToDecode,
    /// Pesistence repository cannot find the record
    NotFound,
}
