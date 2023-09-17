use crate::utils::error::ApiError;
use sqlx::PgPool;
use std::env::var;

pub async fn create_connection() -> Result<PgPool, ApiError> {
    Ok(PgPool::connect(&var("DATABASE_URL")?).await?)
}
