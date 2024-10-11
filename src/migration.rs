use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};
use surrealdb_migrations::MigrationRunner;

pub async fn run() -> std::io::Result<()> {
    let surrel = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();
    surrel
        .signin(Root {
            username: "root",
            password: "root",
        })
        .await
        .unwrap();
    surrel.use_ns("test").use_db("test").await.unwrap();

    MigrationRunner::new(&surrel)
        .up()
        .await
        .expect("Failed to run migrations");

    Ok(())
}
