
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Student {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub mobile: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}
