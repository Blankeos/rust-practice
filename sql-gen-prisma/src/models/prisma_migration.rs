#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PrismaMigration {
    id: String,
    checksum: String,
    finished_at: Option<chrono::DateTime<chrono::Utc>>,
    migration_name: String,
    logs: Option<String>,
    rolled_back_at: Option<chrono::DateTime<chrono::Utc>>,
    started_at: chrono::DateTime<chrono::Utc>,
    applied_steps_count: i32,
}
