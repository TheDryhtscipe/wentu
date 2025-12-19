use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};
use serde_json::json;
use sqlx::Row;
use std::collections::HashSet;
use uuid::Uuid;
use validator::Validate;

use super::AppState;
use crate::audit;
use crate::models::wentu::SAFE_NAME_REGEX;
use crate::models::{JoinWentuRequest, JoinWentuResponse, UpdatePreferencesRequest};
use serde::Deserialize;

/// Join an existing wentu
pub async fn join_wentu(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Json(req): Json<JoinWentuRequest>,
) -> Result<(StatusCode, Json<JoinWentuResponse>), StatusCode> {
    // Validate input
    req.validate().map_err(|e| {
        tracing::warn!("Validation failed for join_wentu: {:?}", e);
        StatusCode::BAD_REQUEST
    })?;

    // Sanitize input
    let name = req.name.trim().to_string();

    if name.is_empty() {
        tracing::warn!("Empty name after trimming");
        return Err(StatusCode::BAD_REQUEST);
    }

    if !SAFE_NAME_REGEX.is_match(&name) {
        tracing::warn!("Invalid characters in participant name");
        return Err(StatusCode::BAD_REQUEST);
    }

    // Fetch wentu
    let wentu_row = sqlx::query("SELECT id FROM wentus WHERE slug = $1")
        .bind(&slug)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let wentu_id: Uuid = wentu_row.get(0);

    let existing = sqlx::query(
        "SELECT p.id, p.participant_key, p.token_expires_at, COUNT(r.id) AS ranking_count
         FROM participants p
         LEFT JOIN rankings r ON r.participant_id = p.id
         WHERE p.wentu_id = $1 AND LOWER(p.name) = LOWER($2)
         GROUP BY p.id
         ORDER BY ranking_count DESC, p.joined_at ASC
         LIMIT 1",
    )
    .bind(wentu_id)
    .bind(&name)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(row) = existing {
        let participant_id: Uuid = row.get("id");
        let participant_key: String = row.get("participant_key");
        let token_expires_at: DateTime<Utc> = row.get("token_expires_at");

        if token_expires_at < Utc::now() {
            let refreshed = Utc::now() + chrono::Duration::days(7);
            sqlx::query("UPDATE participants SET token_expires_at = $1 WHERE id = $2")
                .bind(refreshed)
                .bind(participant_id)
                .execute(&state.db)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        }

        let response = JoinWentuResponse {
            participant_id,
            participant_key: participant_key.clone(),
        };

        audit::log_action(
            &state.db,
            "JOIN_WENTU",
            "participant",
            Some(participant_id),
            Some(&participant_key),
            Some(json!({ "slug": slug, "name": name })),
            true,
        )
        .await;

        return Ok((StatusCode::OK, Json(response)));
    }
    let participant_id = Uuid::new_v4();
    let participant_key = Uuid::new_v4().to_string();
    let token_expires_at = Utc::now() + chrono::Duration::days(7);

    // Insert participant
    sqlx::query(
        "INSERT INTO participants (id, wentu_id, name, participant_key, is_creator, joined_at, token_expires_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
    )
    .bind(participant_id)
    .bind(wentu_id)
    .bind(&name)
    .bind(&participant_key)
    .bind(false)
    .bind(Utc::now())
    .bind(token_expires_at)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = JoinWentuResponse {
        participant_id,
        participant_key: participant_key.clone(),
    };

    audit::log_action(
        &state.db,
        "JOIN_WENTU",
        "participant",
        Some(participant_id),
        Some(&participant_key),
        Some(json!({ "slug": slug, "name": name })),
        true,
    )
    .await;

    Ok((StatusCode::CREATED, Json(response)))
}

/// Update participant preferences
pub async fn update_preferences(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Json(req): Json<UpdatePreferencesRequest>,
) -> Result<StatusCode, StatusCode> {
    // Validate payload
    req.validate().map_err(|e| {
        tracing::warn!("Validation failed for update_preferences: {:?}", e);
        StatusCode::BAD_REQUEST
    })?;

    // Ensure rankings are unique and sequential enough
    let mut seen_options = HashSet::new();
    let mut seen_orders = HashSet::new();
    for ranking in &req.rankings {
        if !seen_options.insert(ranking.date_option_id) {
            tracing::warn!("Duplicate date option in rankings");
            return Err(StatusCode::BAD_REQUEST);
        }
        if !seen_orders.insert(ranking.preference_order) {
            tracing::warn!("Duplicate preference order in rankings");
            return Err(StatusCode::BAD_REQUEST);
        }
    }

    // Verify participant and wentu exist
    let participant_row = sqlx::query(
        "SELECT p.wentu_id, p.token_expires_at FROM participants p
         JOIN wentus w ON p.wentu_id = w.id
         WHERE p.id = $1 AND w.slug = $2 AND p.participant_key = $3",
    )
    .bind(req.participant_id)
    .bind(&slug)
    .bind(&req.participant_key)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::UNAUTHORIZED)?;

    let wentu_id: Uuid = participant_row.get(0);
    let token_expires_at: DateTime<Utc> = participant_row.get(1);

    if token_expires_at < Utc::now() {
        tracing::warn!(
            "update_preferences blocked: token expired for participant {}",
            req.participant_id
        );
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Delete old rankings
    sqlx::query("DELETE FROM rankings WHERE participant_id = $1")
        .bind(req.participant_id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Insert new rankings
    for ranking in &req.rankings {
        sqlx::query(
            "INSERT INTO rankings (participant_id, date_option_id, preference_order)
             VALUES ($1, $2, $3)",
        )
        .bind(req.participant_id)
        .bind(ranking.date_option_id)
        .bind(ranking.preference_order)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    audit::log_action(
        &state.db,
        "UPDATE_PREFERENCES",
        "participant",
        Some(req.participant_id),
        Some(&req.participant_key),
        Some(json!({ "slug": slug, "rankings": req.rankings.len() })),
        true,
    )
    .await;

    Ok(StatusCode::OK)
}

#[derive(Deserialize, Validate)]
pub struct CheckVotedRequest {
    pub participant_id: Uuid,
    #[validate(length(min = 1))]
    pub participant_key: String,
}

/// Check if participant has voted
pub async fn has_voted(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Json(req): Json<CheckVotedRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Validate input
    req.validate().map_err(|e| {
        tracing::warn!("Validation failed for has_voted: {:?}", e);
        StatusCode::BAD_REQUEST
    })?;

    // Verify participant and wentu exist
    let participant_row = sqlx::query(
        "SELECT p.id, p.is_creator
         FROM participants p
         JOIN wentus w ON p.wentu_id = w.id
         WHERE p.id = $1 AND w.slug = $2 AND p.participant_key = $3",
    )
    .bind(req.participant_id)
    .bind(&slug)
    .bind(&req.participant_key)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::UNAUTHORIZED)?;

    let is_creator: bool = participant_row.get(1);

    // Check if participant has any rankings
    let has_rankings = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM rankings WHERE participant_id = $1)",
    )
    .bind(req.participant_id)
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "has_voted": has_rankings,
        "is_creator": is_creator,
    })))
}

/// Get list of participants who have voted (creator only)
pub async fn get_voters(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Json(req): Json<CheckVotedRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Validate input
    req.validate().map_err(|e| {
        tracing::warn!("Validation failed for get_voters: {:?}", e);
        StatusCode::BAD_REQUEST
    })?;

    // Verify participant and wentu exist, and check if creator
    let participant_row = sqlx::query(
        "SELECT p.id, p.is_creator
         FROM participants p
         JOIN wentus w ON p.wentu_id = w.id
         WHERE p.id = $1 AND w.slug = $2 AND p.participant_key = $3",
    )
    .bind(req.participant_id)
    .bind(&slug)
    .bind(&req.participant_key)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::UNAUTHORIZED)?;

    let is_creator: bool = participant_row.get(1);

    // Only creators can see the voter list
    if !is_creator {
        return Err(StatusCode::FORBIDDEN);
    }

    // Get wentu ID
    let wentu_row = sqlx::query("SELECT id FROM wentus WHERE slug = $1")
        .bind(&slug)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let wentu_id: Uuid = wentu_row.get(0);

    // Get list of participants who have voted
    let voters = sqlx::query(
        "SELECT DISTINCT p.name
         FROM participants p
         INNER JOIN rankings r ON p.id = r.participant_id
         WHERE p.wentu_id = $1
         ORDER BY p.name",
    )
    .bind(wentu_id)
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .iter()
    .map(|row| row.get::<String, _>(0))
    .collect::<Vec<_>>();

    Ok(Json(json!({
        "voters": voters,
    })))
}
