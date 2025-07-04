use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod create_token;




#[derive(Debug, Deserialize)]
pub struct CreatTokenRequest {
    user_id: Uuid,
    user_agent: String,
    ip_address: String,
    pub token: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    id: Uuid,
    user_id: Uuid,
    token_hash: String,
    expires_at: DateTime<Utc>,
}