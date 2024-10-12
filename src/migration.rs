use surrealdb_migrations::MigrationRunner;

use crate::utils::db::Database;

pub async fn run() -> std::io::Result<()> {
    let db = Database::new().await;

    MigrationRunner::new(&db)
        .up()
        .await
        .expect("Failed to run migrations");

    Ok(())
}
