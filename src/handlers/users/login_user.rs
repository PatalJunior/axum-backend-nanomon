use axum::extract::State;
use axum::Json;


use crate::domain::models::user::{UserError, UserModel};
use crate::handlers::users::{LoginUserRequest, UserResponse};
use crate::infra::errors::InfraError;
use crate::infra::repositories::user_repository;
use crate::utils::{JsonExtractor};
use crate::AppState;


use argon2::{password_hash::{
    PasswordHash,
}, Argon2, PasswordVerifier};


pub async fn login_user(
    State(state): State<AppState>,
    JsonExtractor(login_user): JsonExtractor<LoginUserRequest>,
) -> Result<Json<UserResponse>, UserError> {
    let user = user_repository::find_by_username(&state.pool, login_user.username.clone())
        .await
        .map_err(|db_error| match db_error {
            InfraError::InternalServerError => UserError::InternalServerError,
            InfraError::NotFound => UserError::InvalidCredentials(login_user.username.clone()),
        })?
        .ok_or_else(|| UserError::InvalidCredentials(login_user.username.clone()))?;

    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|_| UserError::InvalidCredentials(login_user.username.clone()))?;

    let argon2 = Argon2::default();
    argon2
        .verify_password(login_user.password.as_bytes(), &parsed_hash)
        .map_err(|_| UserError::InvalidCredentials(login_user.username.clone()))?;
    
    Ok(Json(adapt_user_to_user_response(user)))
}

fn adapt_user_to_user_response(user: UserModel) -> UserResponse {
    UserResponse {
        id: user.id,
        email: user.email,
        username: user.username,
        created_at: user.created_at,
    }
}