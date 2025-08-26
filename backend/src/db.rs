use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

#[derive(Clone)]
pub struct DbPool(pub PgPool);

impl DbPool {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .min_connections(1)
            .acquire_timeout(Duration::from_secs(5))
            .idle_timeout(Duration::from_secs(300))
            .connect(database_url)
            .await?;
        Ok(Self(pool))
    }
    

    pub async fn health_check(&self) -> Result<(), sqlx::Error> {
        let _: (i64,) = sqlx::query_as("SELECT 1").fetch_one(&self.0).await?;
        Ok(())
    }
}
