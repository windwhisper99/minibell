use std::ops::Deref;

use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

use crate::domain::Error;

#[derive(Clone)]
pub struct Database {
    surreal: Surreal<Client>,
}

impl Deref for Database {
    type Target = Surreal<Client>;

    fn deref(&self) -> &Self::Target {
        &self.surreal
    }
}

impl Database {
    pub async fn new() -> Self {
        let surreal = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();
        surreal
            .signin(Root {
                username: "root",
                password: "root",
            })
            .await
            .unwrap();
        surreal.use_ns("test").use_db("test").await.unwrap();

        Self { surreal }
    }
}

impl From<surrealdb::Error> for Error {
    fn from(value: surrealdb::Error) -> Self {
        Error::RepositoryError(value.to_string())
    }
}
