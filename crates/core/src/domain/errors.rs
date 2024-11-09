#[derive(Debug, Clone)]
pub enum Error {
    ItemNotFound,

    InvalidToken,

    Forbidden,

    Internal(String),
}

impl Error {
    pub fn internal(msg: impl ToString) -> Self {
        Self::Internal(msg.to_string())
    }
}
