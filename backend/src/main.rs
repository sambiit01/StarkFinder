
mod db;
use anyhow::Result;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {

    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let pool = crate::db::DbPool::new(&database_url).await?;
    pool.health_check().await?;
    // Add code here to launch server, pass pool to handlers, etc.
    Ok(())
}
