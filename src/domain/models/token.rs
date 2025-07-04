use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use chrono::{DateTime, Utc};
use serde_json::json;
use uuid::Uuid;
use crate::infra::db::schema::tokens::previous_token_id;
use crate::infra::errors::InfraError;

#[derive(Clone, Debug, PartialEq)]
pub struct TokenModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub ip_address: String,
    pub user_agent: String,
    pub replaced_by: Option<Uuid>,
    pub previous_token_id: Option<Uuid>,
}

#[derive(Debug)]
pub enum TokenError {
    InternalServerError,
    NotFound(Uuid),
    InvalidUuid(String),
    InfraError(InfraError),
}


impl IntoResponse for TokenError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::NotFound(id) => (
                StatusCode::NOT_FOUND,
                format!("TokenModel with id {} has not been found", id),
            ),
            Self::InfraError(db_error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal server error: {}", db_error),
            ),
            Self::InvalidUuid(id) => (
                StatusCode::BAD_REQUEST,
                format!("Invalid UUID: {}", id),
                ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal server error"),
            ),
        };
        (
            status,
            Json(
                json!({"resource":"UserModel", "message": err_msg, "happened_at" : chrono::Utc::now() }),
            ),
        )
            .into_response()
    }
}
