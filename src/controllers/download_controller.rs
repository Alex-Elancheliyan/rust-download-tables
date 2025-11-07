use axum::{extract::{Path, State}, response::{IntoResponse,}, http::{HeaderMap, HeaderValue, StatusCode},};
use sqlx::PgPool;
use crate::services::download_service::generate_file;

pub async fn download_students(State(pool): State<PgPool>, Path(file_type): Path<String>,
) -> impl IntoResponse {
    match generate_file(&pool, &file_type).await {
        Ok((bytes, content_type, filename)) => {
            let mut headers = HeaderMap::new();
            headers.insert("Content-Type", HeaderValue::from_static(content_type));
            headers.insert("Content-Disposition",
                HeaderValue::from_str(&format!("attachment; filename=\"{}\"", filename)).unwrap(),
            );
            (headers, bytes).into_response()
        }
        Err(e) => ( StatusCode::INTERNAL_SERVER_ERROR, format!("Error generating file: {}", e), ).into_response(),
    }
}
