use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ranking {
    pub participant_id: Uuid,
    pub date_option_id: Uuid,
    pub preference_order: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePreferencesRequest {
    pub participant_id: Uuid,
    pub participant_key: String,
    pub rankings: Vec<CreateRanking>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRanking {
    pub date_option_id: Uuid,
    pub preference_order: i32,
}
