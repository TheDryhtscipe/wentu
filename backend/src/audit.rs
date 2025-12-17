use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

/// Persist an audit log entry; best-effort (errors are logged but not bubbled).
pub async fn log_action(
    db: &PgPool,
    action: &str,
    entity_type: &str,
    entity_id: Option<Uuid>,
    user_identifier: Option<&str>,
    details: Option<Value>,
    success: bool,
) {
    if let Err(err) = sqlx::query(
        "INSERT INTO audit_logs (action, entity_type, entity_id, user_identifier, details, success)
         VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(action)
    .bind(entity_type)
    .bind(entity_id)
    .bind(user_identifier)
    .bind(details)
    .bind(success)
    .execute(db)
    .await
    {
        tracing::warn!("Failed to write audit log '{}': {:?}", action, err);
    }
}
