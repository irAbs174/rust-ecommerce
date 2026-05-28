use sqlx::postgres::PgPool;

pub async fn init_db(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPool::connect(database_url).await?;
    sqlx::query("SELECT 1").execute(&pool).await?;
    tracing::info!("Database connection successful");
    Ok(pool)
}
