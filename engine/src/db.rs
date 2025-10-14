use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use dotenv::dotenv;

pub async fn init_db() -> Result<PgPool, sqlx::Error> {
    // Load .env variables (optional)
    dotenv().ok();

    // Get DATABASE_URL from environment
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    // Create PostgreSQL connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    Ok(pool)
}
