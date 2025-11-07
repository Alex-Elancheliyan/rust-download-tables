use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

pub async fn init_db() -> Result<PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new().max_connections(5).connect(&database_url).await
}
