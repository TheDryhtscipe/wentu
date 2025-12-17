use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::{PgPool, Row, FromRow};
use uuid::Uuid;
use chrono::{Utc, DateTime, TimeZone};
use chrono_tz::Tz;
use std::str::FromStr;

use crate::models::{
    Wentu, CreateWentuRequest, CreateWentuResponse, DateRange, WentuStatus,
};
use chrono::Duration;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

#[derive(FromRow)]
struct WentuRow {
    id: Uuid,
    slug: String,
    title: String,
    description: Option<String>,
    creator_name: String,
    creator_key: String,
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
    pref_deadline: DateTime<Utc>,
    #[sqlx(rename = "status")]
    status_str: String,
    timezone: Option<String>,
}

#[derive(FromRow)]
struct DateRangeRow {
    id: Uuid,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    label: String,
}

/// Create a new wentu
pub async fn create_wentu(
    State(state): State<AppState>,
    Json(req): Json<CreateWentuRequest>,
) -> Result<(StatusCode, Json<CreateWentuResponse>), StatusCode> {
    let wentu_id = Uuid::new_v4();
    let creator_key = Uuid::new_v4().to_string();
    let creator_participant_id = Uuid::new_v4();
    let creator_participant_key = Uuid::new_v4().to_string();
    let slug = format!("{}-{}", slugify(&req.title), &creator_key[..8]);
    
    let expires_at = Utc::now() + chrono::Duration::days(req.expires_in_days as i64);

    tracing::info!("Creating wentu: {} with slug: {}", wentu_id, slug);

    // Insert wentu
    sqlx::query(
        "INSERT INTO wentus (id, slug, title, description, creator_name, creator_key, created_at, expires_at, pref_deadline, status, timezone)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10::wentu_status, $11)"
    )
    .bind(wentu_id)
    .bind(&slug)
    .bind(&req.title)
    .bind(&req.description)
    .bind(&req.creator_name)
    .bind(&creator_key)
    .bind(Utc::now())
    .bind(expires_at)
    .bind(&req.pref_deadline)
    .bind("open")
    .bind(&req.timezone)
    .execute(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to insert wentu: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Expand date range into individual days/time slots and insert as date options
    if req.enable_time_slots.unwrap_or(false) {
        // Time slot mode: create multiple records per day
        let tz_str = req.timezone.as_ref().ok_or(StatusCode::BAD_REQUEST)?;
        let tz = Tz::from_str(tz_str).map_err(|_| StatusCode::BAD_REQUEST)?;
        let time_slots = req.day_time_slots.as_ref().ok_or(StatusCode::BAD_REQUEST)?;

        let mut current_date = req.date_range_start;
        let mut sort_order = 0;

        while current_date <= req.date_range_end {
            let date_key = current_date.format("%Y-%m-%d").to_string();

            // Get slots for this day (validation: must exist and not be empty)
            let slots = time_slots.get(&date_key).ok_or(StatusCode::BAD_REQUEST)?;
            if slots.is_empty() || slots.len() > 3 {
                return Err(StatusCode::BAD_REQUEST);
            }

            // Create date_range record for each time slot
            for time_str in slots {
                // Parse "HH:MM" format
                let parts: Vec<&str> = time_str.split(':').collect();
                if parts.len() != 2 { return Err(StatusCode::BAD_REQUEST); }
                let hour: u32 = parts[0].parse().map_err(|_| StatusCode::BAD_REQUEST)?;
                let minute: u32 = parts[1].parse().map_err(|_| StatusCode::BAD_REQUEST)?;

                // Create naive datetime in specified timezone
                let naive_date = current_date.date_naive();
                let naive_time = chrono::NaiveTime::from_hms_opt(hour, minute, 0).ok_or(StatusCode::BAD_REQUEST)?;
                let naive_dt = chrono::NaiveDateTime::new(naive_date, naive_time);

                // Convert to UTC for storage
                let tz_dt = tz.from_local_datetime(&naive_dt).single().ok_or(StatusCode::BAD_REQUEST)?;
                let start_utc = tz_dt.with_timezone(&Utc);
                let end_utc = start_utc + Duration::hours(1);

                // Format label: "Mon, Dec 15 @ 10:00 AM"
                let label = format!("{} @ {}",
                    current_date.format("%a, %b %d"),
                    start_utc.with_timezone(&tz).format("%I:%M %p")
                );

                tracing::debug!("Inserting time slot {} for wentu {}", label, wentu_id);

                // Insert date_range record
                sqlx::query(
                    "INSERT INTO date_ranges (id, wentu_id, start_time, end_time, label, sort_order)
                     VALUES ($1, $2, $3, $4, $5, $6)"
                )
                .bind(Uuid::new_v4())
                .bind(wentu_id)
                .bind(start_utc)
                .bind(end_utc)
                .bind(label)
                .bind(sort_order)
                .execute(&state.db)
                .await
                .map_err(|e| {
                    tracing::error!("Failed to insert time slot: {:?}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;

                sort_order += 1;
            }

            current_date = current_date + Duration::days(1);
        }
    } else {
        // Full-day mode: existing logic
        let mut current_date = req.date_range_start;
        let mut day_count = 0;

        while current_date <= req.date_range_end {
            let label = current_date.format("%a, %b %d").to_string();
            let next_day = current_date + Duration::days(1);

            tracing::debug!("Inserting date option {} (day {}) for wentu {}", day_count, label, wentu_id);

            sqlx::query(
                "INSERT INTO date_ranges (id, wentu_id, start_time, end_time, label, sort_order)
                 VALUES ($1, $2, $3, $4, $5, $6)"
            )
            .bind(Uuid::new_v4())
            .bind(wentu_id)
            .bind(current_date)
            .bind(next_day)
            .bind(label)
            .bind(day_count as i32)
            .execute(&state.db)
            .await
            .map_err(|e| {
                tracing::error!("Failed to insert date option: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

            current_date = next_day;
            day_count += 1;
        }
    }
    
    tracing::info!("Successfully created wentu: {}", wentu_id);
    
    // Automatically create a participant entry for the creator so they can vote immediately
    sqlx::query(
        "INSERT INTO participants (id, wentu_id, name, participant_key, is_creator, joined_at)
         VALUES ($1, $2, $3, $4, $5, $6)"
    )
    .bind(creator_participant_id)
    .bind(wentu_id)
    .bind(&req.creator_name)
    .bind(&creator_participant_key)
    .bind(true)
    .bind(Utc::now())
    .execute(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to insert creator participant: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((
        StatusCode::CREATED,
        Json(CreateWentuResponse {
            id: wentu_id,
            slug: slug.clone(),
            link: format!("/wentu/{}", slug),
            creator_key,
            creator_participant_id,
            creator_participant_key,
        }),
    ))
}

/// Get wentu by slug
pub async fn get_wentu(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<Wentu>, StatusCode> {
    tracing::info!("GET wentu: {}", slug);
    
    // Fetch wentu from database
    let wentu_result = sqlx::query_as::<_, WentuRow>(
        "SELECT id, slug, title, description, creator_name, creator_key, created_at, expires_at, pref_deadline, status::text, timezone
         FROM wentus WHERE slug = $1"
    )
    .bind(&slug)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch wentu: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(StatusCode::NOT_FOUND)?;

    // Fetch date options
    let date_rows = sqlx::query_as::<_, DateRangeRow>(
        "SELECT id, start_time, end_time, label FROM date_ranges WHERE wentu_id = $1 ORDER BY sort_order"
    )
    .bind(wentu_result.id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch date ranges: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let mut status = match wentu_result.status_str.as_str() {
        "open" => WentuStatus::Open,
        "closed" => WentuStatus::Closed,
        "expired" => WentuStatus::Expired,
        _ => WentuStatus::Open,
    };

    // Check if expired
    if Utc::now() > wentu_result.expires_at {
        status = WentuStatus::Expired;
    }

    let date_options = date_rows
        .into_iter()
        .map(|row| DateRange {
            id: row.id,
            start: row.start_time,
            end: row.end_time,
            label: row.label,
        })
        .collect();

    let wentu = Wentu {
        id: wentu_result.id,
        slug: wentu_result.slug,
        title: wentu_result.title,
        description: wentu_result.description,
        creator_name: wentu_result.creator_name,
        creator_key: wentu_result.creator_key,
        created_at: wentu_result.created_at,
        expires_at: wentu_result.expires_at,
        pref_deadline: wentu_result.pref_deadline,
        status,
        date_options,
        timezone: wentu_result.timezone,
    };

    Ok(Json(wentu))
}

/// Close wentu early
pub async fn close_wentu(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Json(req): Json<serde_json::Value>,
) -> Result<StatusCode, StatusCode> {
    let creator_key = req
        .get("creator_key")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;

    sqlx::query(
        "UPDATE wentus SET status = 'closed'::wentu_status WHERE slug = $1 AND creator_key = $2"
    )
    .bind(&slug)
    .bind(creator_key)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}

fn slugify(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}
