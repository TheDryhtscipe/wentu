use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::Duration;
use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::Tz;
use serde_json::json;
use sqlx::{FromRow, PgPool, Row};
use std::str::FromStr;
use uuid::Uuid;
use validator::Validate;

use crate::audit;
use crate::models::wentu::{SAFE_NAME_REGEX, SAFE_TITLE_REGEX};
use crate::models::{
    CloseWentuRequest, CreateWentuRequest, CreateWentuResponse, DateRange, Wentu, WentuStatus,
};

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
    // Validate input
    req.validate().map_err(|e| {
        tracing::warn!("Validation failed: {:?}", e);
        StatusCode::BAD_REQUEST
    })?;

    // Additional date range validation
    if req.date_range_start >= req.date_range_end {
        tracing::warn!("Invalid date range: start must be before end");
        return Err(StatusCode::BAD_REQUEST);
    }

    let range_days = (req.date_range_end - req.date_range_start).num_days();
    if range_days > 365 {
        tracing::warn!("Date range too large: {} days", range_days);
        return Err(StatusCode::BAD_REQUEST);
    }

    // Sanitize inputs
    let title = req.title.trim().to_string();
    let creator_name = req.creator_name.trim().to_string();
    let description = req.description.as_ref().map(|d| d.trim().to_string());

    if !SAFE_TITLE_REGEX.is_match(&title) {
        tracing::warn!("Invalid characters in title");
        return Err(StatusCode::BAD_REQUEST);
    }

    if !SAFE_NAME_REGEX.is_match(&creator_name) {
        tracing::warn!("Invalid characters in creator name");
        return Err(StatusCode::BAD_REQUEST);
    }

    let wentu_id = Uuid::new_v4();
    let creator_key = Uuid::new_v4().to_string();
    let creator_participant_id = Uuid::new_v4();
    let creator_participant_key = Uuid::new_v4().to_string();
    let slug = format!("{}-{}", slugify(&title), &creator_key[..8]);

    // Wentu expires 7 days AFTER the voting deadline, not after creation
    let expires_at = req.pref_deadline + chrono::Duration::days(7);

    tracing::info!("Creating wentu: {} with slug: {}", wentu_id, slug);

    // Insert wentu
    sqlx::query(
        "INSERT INTO wentus (id, slug, title, description, creator_name, creator_key, created_at, expires_at, pref_deadline, status, timezone)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10::wentu_status, $11)"
    )
    .bind(wentu_id)
    .bind(&slug)
    .bind(&title)
    .bind(&description)
    .bind(&creator_name)
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

        if time_slots.is_empty() {
            return Err(StatusCode::BAD_REQUEST);
        }

        let min_date = req.date_range_start.date_naive();
        let max_date = req.date_range_end.date_naive();

        let mut date_keys: Vec<String> = time_slots.keys().cloned().collect();
        date_keys.sort();

        let mut sort_order = 0;

        for date_key in date_keys {
            let date = chrono::NaiveDate::parse_from_str(&date_key, "%Y-%m-%d")
                .map_err(|_| StatusCode::BAD_REQUEST)?;
            if date < min_date || date > max_date {
                return Err(StatusCode::BAD_REQUEST);
            }

            // Get slots for this day (validation: must exist and not be empty)
            let slots = time_slots.get(&date_key).ok_or(StatusCode::BAD_REQUEST)?;
            if slots.is_empty() || slots.len() > 3 {
                return Err(StatusCode::BAD_REQUEST);
            }

            // Create date_range record for each time slot
            for time_str in slots {
                // Parse "HH:MM" format
                let parts: Vec<&str> = time_str.split(':').collect();
                if parts.len() != 2 {
                    return Err(StatusCode::BAD_REQUEST);
                }
                let hour: u32 = parts[0].parse().map_err(|_| StatusCode::BAD_REQUEST)?;
                let minute: u32 = parts[1].parse().map_err(|_| StatusCode::BAD_REQUEST)?;

                // Create naive datetime in specified timezone
                let naive_time = chrono::NaiveTime::from_hms_opt(hour, minute, 0)
                    .ok_or(StatusCode::BAD_REQUEST)?;
                let naive_dt = chrono::NaiveDateTime::new(date, naive_time);

                // Convert to UTC for storage
                let tz_dt = tz
                    .from_local_datetime(&naive_dt)
                    .single()
                    .ok_or(StatusCode::BAD_REQUEST)?;
                let start_utc = tz_dt.with_timezone(&Utc);
                let end_utc = start_utc + Duration::hours(1);

                // Format label: "Mon, Dec 15 @ 10:00 AM"
                let label = format!(
                    "{} @ {}",
                    date.format("%a, %b %d"),
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
        }
    } else {
        // Full-day mode: existing logic
        let mut current_date = req.date_range_start;
        let mut day_count = 0;

        while current_date <= req.date_range_end {
            let label = current_date.format("%a, %b %d").to_string();
            let next_day = current_date + Duration::days(1);

            tracing::debug!(
                "Inserting date option {} (day {}) for wentu {}",
                day_count,
                label,
                wentu_id
            );

            sqlx::query(
                "INSERT INTO date_ranges (id, wentu_id, start_time, end_time, label, sort_order)
                 VALUES ($1, $2, $3, $4, $5, $6)",
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
    let creator_token_expires = Utc::now() + Duration::days(7);

    sqlx::query(
        "INSERT INTO participants (id, wentu_id, name, participant_key, is_creator, joined_at, token_expires_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
    )
    .bind(creator_participant_id)
    .bind(wentu_id)
    .bind(&req.creator_name)
    .bind(&creator_participant_key)
    .bind(true)
    .bind(Utc::now())
    .bind(creator_token_expires)
    .execute(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to insert creator participant: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    audit::log_action(
        &state.db,
        "CREATE_WENTU",
        "wentu",
        Some(wentu_id),
        Some(&creator_key),
        Some(json!({ "slug": slug.clone(), "title": title })),
        true,
    )
    .await;

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
    Json(req): Json<CloseWentuRequest>,
) -> Result<StatusCode, StatusCode> {
    req.validate().map_err(|e| {
        tracing::warn!("close_wentu validation failed: {:?}", e);
        StatusCode::BAD_REQUEST
    })?;

    let creator_key_trimmed = req.creator_key.trim();
    let creator_key = Uuid::parse_str(creator_key_trimmed)
        .map_err(|_| {
            tracing::warn!("close_wentu invalid key format for slug {}", slug);
            StatusCode::BAD_REQUEST
        })?
        .to_string();

    let wentu_row = sqlx::query("SELECT id, status::text, creator_key FROM wentus WHERE slug = $1")
        .bind(&slug)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch wentu {}: {:?}", slug, e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    let wentu_id: Uuid = wentu_row.get(0);
    let current_status: String = wentu_row.get(1);
    let stored_creator_key: String = wentu_row.get(2);

    match current_status.as_str() {
        "closed" => {
            tracing::info!("Wentu {} already closed", slug);
            return Err(StatusCode::CONFLICT);
        }
        "expired" => {
            tracing::info!("Wentu {} already expired", slug);
            return Err(StatusCode::FORBIDDEN);
        }
        _ => {}
    }

    if stored_creator_key != creator_key {
        tracing::warn!("close_wentu unauthorized for slug {}", slug);
        audit::log_action(
            &state.db,
            "CLOSE_WENTU",
            "wentu",
            Some(wentu_id),
            Some(creator_key_trimmed),
            Some(json!({ "slug": slug.clone(), "reason": "unauthorized" })),
            false,
        )
        .await;
        return Err(StatusCode::UNAUTHORIZED);
    }

    sqlx::query("UPDATE wentus SET status = 'closed'::wentu_status WHERE id = $1")
        .bind(wentu_id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to close wentu {}: {:?}", slug, e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    tracing::info!("Wentu {} closed successfully", slug);
    audit::log_action(
        &state.db,
        "CLOSE_WENTU",
        "wentu",
        Some(wentu_id),
        Some(creator_key_trimmed),
        Some(json!({ "slug": slug })),
        true,
    )
    .await;
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
