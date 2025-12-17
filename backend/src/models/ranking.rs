use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ranking {
    pub participant_id: Uuid,
    pub date_option_id: Uuid,
    pub preference_order: i32,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdatePreferencesRequest {
    pub participant_id: Uuid,
    #[validate(length(min = 1, max = 255, message = "Participant key required"))]
    pub participant_key: String,
    #[validate(length(min = 1, max = 10, message = "Provide 1-10 rankings"))]
    #[validate(nested)]
    pub rankings: Vec<CreateRanking>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateRanking {
    pub date_option_id: Uuid,
    #[validate(range(min = 1, max = 10, message = "Preference order must be 1-10"))]
    pub preference_order: i32,
}
