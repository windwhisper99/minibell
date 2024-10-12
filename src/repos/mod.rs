pub mod member;

#[derive(Debug)]
pub enum RepoError {
    NotFound,
    DatabaseError(surrealdb::Error),
}

impl From<surrealdb::Error> for RepoError {
    fn from(value: surrealdb::Error) -> Self {
        RepoError::DatabaseError(value)
    }
}
