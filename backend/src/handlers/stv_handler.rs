use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::{FromRow, Row};
use uuid::Uuid;

use super::AppState;
use crate::stv::{calculate_stv, STVResult};

#[derive(FromRow)]
struct ParticipantPreferences {
    participant_id: Uuid,
    date_option_id: Uuid,
    preference_order: i32,
}

#[derive(FromRow)]
struct DateOptionInfo {
    id: Uuid,
}

/// Get STV results for a wentu
pub async fn get_stv_results(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    tracing::info!("GET STV results for: {}", slug);

    // Get wentu ID
    let wentu_row = sqlx::query("SELECT id FROM wentus WHERE slug = $1")
        .bind(&slug)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch wentu: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    let wentu_id: Uuid = wentu_row.get(0);

    // Get all date options in order
    let date_options = sqlx::query_as::<_, DateOptionInfo>(
        "SELECT id FROM date_ranges WHERE wentu_id = $1 ORDER BY sort_order",
    )
    .bind(wentu_id)
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .into_iter()
    .map(|r| r.id)
    .collect::<Vec<_>>();

    // Get all participant preferences
    let preferences = sqlx::query_as::<_, ParticipantPreferences>(
        "SELECT participant_id, date_option_id, preference_order 
         FROM rankings 
         WHERE date_option_id IN (SELECT id FROM date_ranges WHERE wentu_id = $1)
         ORDER BY participant_id, preference_order",
    )
    .bind(wentu_id)
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Build voter preferences structure (participant -> ordered date options)
    let mut voter_prefs: std::collections::HashMap<Uuid, Vec<Uuid>> =
        std::collections::HashMap::new();
    for pref in preferences {
        voter_prefs
            .entry(pref.participant_id)
            .or_insert_with(Vec::new)
            .push(pref.date_option_id);
    }

    // Convert to list of preference vectors
    let voter_preferences = voter_prefs.into_iter().map(|(_, prefs)| prefs).collect();

    // Calculate STV
    let result = calculate_stv(voter_preferences, date_options);

    // Build response
    let response = serde_json::json!({
        "winner": result.winner,
        "quota": result.quota,
        "rounds_count": result.rounds.len(),
        "rounds": result.rounds.iter().map(|round| {
            serde_json::json!({
                "round_number": round.round_number,
                "vote_counts": round.vote_counts,
                "eliminated": round.eliminated,
            })
        }).collect::<Vec<_>>(),
    });

    Ok(Json(response))
}
