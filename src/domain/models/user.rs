use axum::http::StatusCode;
use argon2::password_hash::Error as PasswordHashError;
use axum::Json;
use axum::response::IntoResponse;
use chrono::NaiveDate;
use serde_json::json;
use uuid::Uuid;

use crate::infra::errors::InfraError;

#[derive(Clone, Debug, PartialEq)]
pub struct UserModel {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub is_admin: bool,
    pub created_at: NaiveDate,
}

#[derive(Debug)]
pub enum UserError {
    InternalServerError,
    NotFound(Uuid),
    InvalidCredentials(String),
    PasswordHashError(PasswordHashError),
    InfraError(InfraError),
}

impl From<PasswordHashError> for UserError {
    fn from(err: PasswordHashError) -> Self {
        UserError::PasswordHashError(err)
    }
}

impl IntoResponse for UserError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::NotFound(id) => (
                StatusCode::NOT_FOUND,
                format!("UserModel with id {} has not been found", id),
            ),
            Self::InvalidCredentials(username) => (
                StatusCode::UNAUTHORIZED,
                format!("User with username {} and provided password has not been found", username),
            ),
            Self::InfraError(db_error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal server error: {}", db_error),
            ),
            Self::PasswordHashError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal server error: {}", err),
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
