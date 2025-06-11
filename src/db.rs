use sqlx::PgPool;

pub async fn get_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    PgPool::connect(&database_url).await
}
