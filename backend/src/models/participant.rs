use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    pub id: Uuid,
    pub wentu_id: Uuid,
    pub name: String,
    pub participant_key: String,
    pub is_creator: bool,
    pub joined_at: DateTime<Utc>,
    pub token_expires_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct JoinWentuRequest {
    #[validate(length(min = 1, max = 100, message = "Name must be 1-100 characters"))]
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct JoinWentuResponse {
    pub participant_id: Uuid,
    pub participant_key: String,
}
