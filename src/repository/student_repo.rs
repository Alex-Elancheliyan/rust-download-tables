
use sqlx::PgPool;
use crate::models::signup::Signup;

pub async fn get_all_students(pool: &PgPool) -> Result<Vec<Signup>, sqlx::Error> {
    let signups = sqlx::query_as::<_, Signup>(
        r#"SELECT id, name, email, mobile, password, created_at FROM signup ORDER BY id"#
    ).fetch_all(pool).await?;

    Ok(signups)
}
