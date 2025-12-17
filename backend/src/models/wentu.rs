use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "wentu_status", rename_all = "lowercase")]
pub enum WentuStatus {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "expired")]
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub id: Uuid,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wentu {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub creator_name: String,
    pub creator_key: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub pref_deadline: DateTime<Utc>,
    pub status: WentuStatus,
    pub date_options: Vec<DateRange>,
    pub timezone: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateWentuRequest {
    #[validate(length(min = 1, max = 255, message = "Title must be 1-255 characters"))]
    pub title: String,

    #[validate(length(max = 2000, message = "Description must be under 2000 characters"))]
    pub description: Option<String>,

    #[validate(length(min = 1, max = 100, message = "Creator name must be 1-100 characters"))]
    pub creator_name: String,

    pub date_range_start: DateTime<Utc>,
    pub date_range_end: DateTime<Utc>,
    pub pref_deadline: DateTime<Utc>,

    #[validate(range(min = 1, max = 365, message = "Expiration must be 1-365 days"))]
    pub expires_in_days: i32,

    // Time slot configuration
    pub enable_time_slots: Option<bool>,
    pub timezone: Option<String>,
    pub day_time_slots: Option<HashMap<String, Vec<String>>>,
}

#[derive(Debug, Serialize)]
pub struct CreateWentuResponse {
    pub id: Uuid,
    pub slug: String,
    pub link: String,
    pub creator_key: String,
    pub creator_participant_id: Uuid,
    pub creator_participant_key: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CloseWentuRequest {
    #[validate(length(min = 32, max = 64, message = "creator_key must be provided"))]
    pub creator_key: String,
}

lazy_static! {
    pub static ref SAFE_TITLE_REGEX: Regex = Regex::new("^[a-zA-Z0-9\\s\\-_'\".,!?()]+$").unwrap();
    pub static ref SAFE_NAME_REGEX: Regex = Regex::new("^[a-zA-Z0-9\\s\\-_'\".]+$").unwrap();
}
