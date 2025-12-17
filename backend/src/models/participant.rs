use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    pub id: Uuid,
    pub wentu_id: Uuid,
    pub name: String,
    pub participant_key: String,
    pub is_creator: bool,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct JoinWentuRequest {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct JoinWentuResponse {
    pub participant_id: Uuid,
    pub participant_key: String,
}
