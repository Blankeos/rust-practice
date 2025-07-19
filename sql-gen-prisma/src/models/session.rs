#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Session {
    id: String,
    user_id: String,
    expires_at: chrono::NaiveDateTime,
}
