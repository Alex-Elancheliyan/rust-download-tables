use axum::{Router, routing::get};
use sqlx::PgPool;
use crate::controllers::download_controller::download_students;

pub fn create_download_routes(pool: PgPool) -> Router {
    Router::new().route("/download/:file_type", get(download_students)).with_state(pool)  // ✅ Pass the DB connection
}
