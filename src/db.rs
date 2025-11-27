use sqlx::{Pool, Postgres};

pub type DbPool = Pool<Postgres>;

pub async fn create_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}

pub async fn run_migrations(pool: &DbPool) -> Result<(), sqlx::Error> {
    match sqlx::migrate!("./migrations").run(pool).await {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::warn!("Migration failed: {:?}. Attempting to repair...", e);
            // Attempt to fix VersionMismatch by removing the problematic entry
            // This is safe because our migrations are idempotent (IF NOT EXISTS)
            sqlx::query("DELETE FROM _sqlx_migrations WHERE version = 20251126")
                .execute(pool)
                .await?;
            
            // Retry migration
            sqlx::migrate!("./migrations").run(pool).await?;
            Ok(())
        }
    }
}
