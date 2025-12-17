use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

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

#[derive(Debug, Deserialize)]
pub struct CreateWentuRequest {
    pub title: String,
    pub description: Option<String>,
    pub creator_name: String,
    pub date_range_start: DateTime<Utc>,
    pub date_range_end: DateTime<Utc>,
    pub pref_deadline: DateTime<Utc>,
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
