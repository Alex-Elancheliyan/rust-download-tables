use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Signup {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub mobile: Option<String>,
    pub password: String,
    pub created_at: Option<NaiveDateTime>,
}
