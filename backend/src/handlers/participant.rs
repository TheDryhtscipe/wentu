use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono::Utc;

use crate::models::{JoinWentuRequest, JoinWentuResponse, UpdatePreferencesRequest};
use super::AppState;

/// Join an existing wentu
pub async fn join_wentu(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Json(req): Json<JoinWentuRequest>,
) -> Result<(StatusCode, Json<JoinWentuResponse>), StatusCode> {
    // Fetch wentu
    let wentu_row = sqlx::query("SELECT id FROM wentus WHERE slug = $1")
        .bind(&slug)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let wentu_id: Uuid = wentu_row.get(0);
    let participant_id = Uuid::new_v4();
    let participant_key = Uuid::new_v4().to_string();

    // Insert participant
    sqlx::query(
        "INSERT INTO participants (id, wentu_id, name, participant_key, is_creator, joined_at)
         VALUES ($1, $2, $3, $4, $5, $6)"
    )
    .bind(participant_id)
    .bind(wentu_id)
    .bind(&req.name)
    .bind(&participant_key)
    .bind(false)
    .bind(Utc::now())
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((
        StatusCode::CREATED,
        Json(JoinWentuResponse {
            participant_id,
            participant_key,
        }),
    ))
}

/// Update participant preferences
pub async fn update_preferences(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Json(req): Json<UpdatePreferencesRequest>,
) -> Result<StatusCode, StatusCode> {
    // Verify participant and wentu exist
    let participant_row = sqlx::query(
        "SELECT p.wentu_id FROM participants p
         JOIN wentus w ON p.wentu_id = w.id
         WHERE p.id = $1 AND w.slug = $2 AND p.participant_key = $3"
    )
    .bind(req.participant_id)
    .bind(&slug)
    .bind(&req.participant_key)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::UNAUTHORIZED)?;

    let wentu_id: Uuid = participant_row.get(0);

    // Delete old rankings
    sqlx::query("DELETE FROM rankings WHERE participant_id = $1")
        .bind(req.participant_id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Insert new rankings
    for ranking in req.rankings {
        sqlx::query(
            "INSERT INTO rankings (participant_id, date_option_id, preference_order)
             VALUES ($1, $2, $3)"
        )
        .bind(req.participant_id)
        .bind(ranking.date_option_id)
        .bind(ranking.preference_order)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(StatusCode::OK)
}
