use sqlx::PgPool;

pub type Db = PgPool;

pub async fn init_pool() -> anyhow::Result<Db> {
    let url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&url).await?;
    // quick check
    sqlx::query!("SELECT 1").execute(&pool).await?;
    Ok(pool)
}
